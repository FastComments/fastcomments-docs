## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filter | string | No |  |
| searchFilters | string | No |  |
| demo | boolean | No |  |
| sso | string | No |  |

## Response

Returns: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationAPICountCommentsResponse.ts)

## Example

[inline-code-attrs-start title = 'getCount Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const countResult: ModerationAPICountCommentsResponse = await getCount(
    "reported harassment",
    "203.0.113.7",
    "status:pending",
    undefined,
    false,
    "sso_user_789.jwt.token"
  );
  console.log(countResult);
})();
[inline-code-end]
