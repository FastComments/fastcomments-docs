---
## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| comment_id | String | Ja |  |
| sso | String | Nej |  |

## Respons

Returnerer: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/post_remove_comment_response.rs)

## Eksempel

[inline-code-attrs-start title = 'post_remove_comment Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_remove_comment() -> Result<PostRemoveCommentResponse, Error> {
    let params: PostRemoveCommentParams = PostRemoveCommentParams {
        comment_id: String::from("cmt-9f8b6a3"),
        sso: Some(String::from("sso-token-6f4e9a2b")),
    };
    let response: PostRemoveCommentResponse = post_remove_comment(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---