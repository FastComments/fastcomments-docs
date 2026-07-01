## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|-------------|------|
| tag | string | Так |  |
| tenantId | string | Ні |  |
| updateHashTagBody | UpdateHashTagBody | Ні |  |

## Відповідь

Повертає: [`PatchHashTagResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PatchHashTagResponse.ts)

## Приклад

[inline-code-attrs-start title = 'patchHashTag Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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