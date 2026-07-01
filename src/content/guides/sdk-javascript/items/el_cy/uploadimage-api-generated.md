Upload and resize an image

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| file | Blob | Ναι |  |
| sizePreset | SizePreset | Όχι |  |
| urlId | string | Όχι |  |

## Απόκριση

Επιστρέφει: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UploadImageResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα uploadImage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-01";
const imageBlob: Blob = new Blob([new Uint8Array([137,80,78,71])], { type: "image/png" });

const uploadResult1: UploadImageResponse = await uploadImage(tenantId, imageBlob);

const sizePreset: SizePreset = { presetName: "medium" };
const urlId: string = "article-9876";

const uploadResult2: UploadImageResponse = await uploadImage(tenantId, imageBlob, sizePreset, urlId);
[inline-code-end]