---
A `Moderator` オブジェクトはモデレーターの設定を表します。

モデレーターには3つのタイプがあります:

1. `isCommentModeratorAdmin` フラグを持つ管理者ユーザー。
2. `isCommentModeratorAdmin` フラグを持つSSOユーザー。
3. 招待された通常のコメント投稿者、またはFastComments.comのユーザー。

`Moderator` 構造はユースケース `3` のモデレーション状態を表すために使用されます。

ユーザーをモデレーターとしてAPI経由で招待したい場合は、`Moderator` APIを使用して `Moderator` を作成し、彼らを `inviting` してください。

ユーザーがFastComments.comアカウントを持っていない場合、招待メールがセットアップを手助けします。すでにアカウントを持っている場合は、テナントに対するモデレーションアクセスが付与され、`Moderator` オブジェクトの `userId` がそのユーザーを指すように更新されます。この場合、そのユーザーのAPIアクセスはあなたにはなく、FastComments.comによって管理されます。

ユーザーアカウントを完全に管理する必要がある場合は、SSOを使用するか、彼らを[テナントユーザー](https://fastcomments.com/auth/my-account/users)として追加し、その後に `Moderator` オブジェクトを追加して統計を追跡することをおすすめします。

`Moderator` 構造はユースケース `1` と `2` の統計追跡メカニズムとして使用できます。ユーザーを作成した後、`userId` を定義した `Moderator` オブジェクトを追加すると、彼らの統計が[コメントモデレーターのページ](https://fastcomments.com/auth/my-account/moderate-comments/moderators)で追跡されます。

`Moderator` オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'Moderator 構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Moderator {
    name: string
    email: string
    tenantId: string
    userId?: string|null
    acceptedInvite?: boolean
    markReviewedCount?: number
    deletedCount?: number
    markedSpamCount?: number
    approvedCount?: number
    editedCount?: number
    bannedCount?: number
    createdAt: string
    moderationGroupIds?: string[]|null
}
[inline-code-end]

---