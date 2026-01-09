FastComments works with membership-only sites by using what is called SSO, or single-sign-on. Your members sign in to your WordPress site, but
댓글을 달기 위해 FastComments 계정을 만들거나 소셜 미디어로 로그인하는 것에 대해 걱정할 필요가 없습니다. 귀하의 회원들(관리자 포함)이 귀하의 WordPress 사이트에 로그인해 있으면 댓글을 작성할 수 있습니다. 관리자와 중재자는 WordPress 블로그 게시물에서 직접 댓글을 관리할 수도 있습니다.

<sup>(Optional)</sup> 관리자들의 경험을 향상시키고 중재자에 대한 통계 추적을 활성화하려면 관리자들을 [User & Administrators](https://fastcomments.com/auth/my-account/users)와 중재자들을 [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
에 추가하는 것을 잊지 마십시오.

SSO can be enabled by going to the plugin dashboard and clicking "SSO Settings".

You will first have to enable the "Anyone can Register" feature of your site.

All user information is seamlessly and securely transferred to FastComments each time a user views a comment thread via [HMAC](https://en.wikipedia.org/wiki/HMAC).

There is no initial or continuous sync that needs to be run to copy your members over to FastComments servers. This is automatically done when they view comment threads by opening a blog post.

## Names and Avatars

The plugin will automatically update the user's display name and avatar on all their comments within FastComments when they view
any comment thread. 아바타는 Gravatar 또는 WordPress 내의 다른 아바타 관리 플러그인을 통해 지원되며, 플러그인은 `get_avatar_url`을 호출합니다.

---