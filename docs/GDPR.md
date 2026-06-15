# GDPR Compliance Features

Stellar Deepdive includes a set of GDPR-oriented features for managing user consent, exporting personal data, and requesting account/data deletion. This document describes the frontend flows, the API contract they expect, and the current backend status.

---

## Frontend Components

All components live in `frontend/src/components/gdpr/` and are exported from its `index.ts`:

```ts
import { ConsentManager, DataExport, DataDeletion } from "@/components/gdpr";
```

They share a common pattern: each loads its state on mount via `frontend/src/lib/gdpr-api.ts`, shows a loading spinner, and surfaces errors/success messages inline.

### ConsentManager

Displays a toggle for each consent type (see **Consent Types** below) using `getConsents()`, and saves changes in one batch via `batchUpdateConsents()`. Labels and descriptions for each consent type come from `CONSENT_LABELS` in `gdpr-api.ts`.

### DataExport

Lets users select which categories of their data to export (from `getExportableTypes()`), then creates an export request via `createExportRequest({ data_types, export_format })`. A history tab lists past requests (`getExportRequests()`) with status badges and a download link once `status === "completed"`.

### DataDeletion

Lets users request deletion of all their data (or, in future, specific data types — currently marked "coming soon" in the UI). Flow:

1. User selects scope and optional reason, then confirms via a two-step confirmation dialog.
2. `createDeletionRequest({ reason, delete_all_data })` is called.
3. If the response has `confirmation_required: true`, the UI shows the `confirmation_token` and tells the user they have 24 hours to confirm.
4. A history tab (`getDeletionRequests()`) lists requests with status badges; pending/scheduled requests can be cancelled via `cancelDeletionRequest(id)`.

---

## Consent Types

Defined in `CONSENT_LABELS` (`frontend/src/lib/gdpr-api.ts`):

| `consent_type` | Title | Description |
|---|---|---|
| `terms_of_service` | Terms of Service | I agree to the Terms of Service |
| `privacy_policy` | Privacy Policy | I have read and agree to the Privacy Policy |
| `marketing_emails` | Marketing Emails | Receive emails about product updates and promotions |
| `analytics` | Analytics | Allow usage analytics to improve the service |
| `personalization` | Personalization | Allow personalized recommendations and features |
| `data_sharing` | Data Sharing | Share data with partners for improved services |
| `cookies` | Cookies | Allow cookies for session management and analytics |

---

## API Contract

The frontend expects the following endpoints under `/api/gdpr`, all authenticated via `Authorization: Bearer <access_token>`:

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/api/gdpr/consents` | List the current user's consent records. |
| `PUT` | `/api/gdpr/consents` | Update a single consent. |
| `PUT` | `/api/gdpr/consents/batch` | Update multiple consents at once. |
| `GET` | `/api/gdpr/export` | List the user's data export requests. |
| `POST` | `/api/gdpr/export` | Create a new export request for the given `data_types`. |
| `GET` | `/api/gdpr/export/:id` | Get the status/download URL of a specific export. |
| `GET` | `/api/gdpr/export-types` | List exportable data categories. |
| `GET` | `/api/gdpr/deletion` | List the user's deletion requests. |
| `POST` | `/api/gdpr/deletion` | Create a deletion request (optionally scoped). |
| `GET` | `/api/gdpr/deletion/:id` | Get the status of a specific deletion request. |
| `POST` | `/api/gdpr/deletion/:id/cancel` | Cancel a pending/scheduled deletion request. |
| `POST` | `/api/gdpr/deletion/confirm` | Confirm a deletion using its `confirmation_token`. |
| `GET` | `/api/gdpr/summary` | Aggregate summary: consents, pending export/deletion counts, processing activity count. |

### Deletion Request Lifecycle

`pending` → (`confirmation_required` → user confirms with token) → `scheduled` → `processing` → `completed`, or `cancelled`/`failed` at any point before completion. Scheduled deletions carry a `scheduled_deletion_at` timestamp (e.g. a grace period before permanent removal).

### Export Request Lifecycle

`pending` → `processing` → `completed` (with `download_url` and `expires_at`), or `failed`/`expired`.

---

## Backend Status

The corresponding backend module (`backend/src/gdpr`) and its route registrations in `backend/src/main.rs` are currently **commented out** pending completion — the routes above describe the intended contract that the frontend is built against. Re-enabling GDPR support requires:

1. Implementing `backend/src/gdpr` (service + handlers) matching the response shapes in `frontend/src/lib/gdpr-api.ts`.
2. Uncommenting the `gdpr_service` initialization and `gdpr_routes` merge in `backend/src/main.rs`.
3. Adding the relevant database migrations for consents, export requests, and deletion requests.
