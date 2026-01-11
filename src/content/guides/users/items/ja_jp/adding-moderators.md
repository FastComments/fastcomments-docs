モデレーターの追加に関するドキュメントは[こちら](/guide-moderation.html#moderators-adding)をご覧ください。

Note that unlike `Administrators`, `Moderator` permissions are fixed - they cannot be changed. Moderators only have access
to the Moderate Comments page. They cannot change settings.

While it is possible to simply add an `Administrator` with the `Moderate Comments` permission, there are some benefits
to utilizing the moderator-specific users:

1. 現在、コメントのモデレーション統計（承認されたコメント、削除、編集、利用者のバンなど）は`Moderators`に関するもののみ提供しています。
2. それらはすべて、別個の1箇所で管理できます。

`Administrators` can manage and view stats around `Moderators` [here](https://fastcomments.com/auth/my-account/moderate-comments/moderators).

If you would like to add a user whom primarily does comment moderation, but is able to do other
things with the account, consider creating an `Administrator` and customizing their permissions. However, you will
not have access to statistics regarding their moderation performance.