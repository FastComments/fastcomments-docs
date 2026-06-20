## Parametre

| Name | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| commentId | string | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Eksempel

[inline-code-attrs-start title = 'postRestoreDeletedComment Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postRestoreDeletedComment(commentId = "comment-8a7b6c5d", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.SAMPLE_SIGNATURE")
if response.isSome:
  let apiResp = response.get()
  echo "Comment restored:", apiResp
else:
  echo "Restore request failed"
[inline-code-end]