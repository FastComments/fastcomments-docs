## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tag | string | Da |  |
| tenantId | string | Ne |  |
| updateHashTagBody | UpdateHashTagBody | Ne |  |

## Odgovor

Vraća: [`PatchHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTagResponse.ts)

## Primjer

[inline-code-attrs-start title = 'patchHashTag Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const response1: PatchHashTagResponse = await patchHashTag("new-feature");

const response2: PatchHashTagResponse = await patchHashTag(
  "beta-release",
  "tenant-9f8b7c"
);

const updateBody: UpdateHashTagBody = {
  description: "Mark comments related to the upcoming beta release",
  color: "#1e90ff"
};

const response3: PatchHashTagResponse = await patchHashTag(
  "beta-release",
  "tenant-9f8b7c",
  updateBody
);
[inline-code-end]