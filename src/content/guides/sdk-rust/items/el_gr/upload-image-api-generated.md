---
Μεταφόρτωση και αλλαγή μεγέθους εικόνας

## Παράμετροι

| Όνομα | Τύπος | Υποχρεωτικό | Περιγραφή |
|------|------|----------|-------------|
| tenant_id | String | Ναι |  |
| file | std::path::PathBuf | Ναι |  |
| size_preset | models::SizePreset | Όχι |  |
| url_id | String | Όχι |  |

## Απόκριση

Επιστρέφει: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---