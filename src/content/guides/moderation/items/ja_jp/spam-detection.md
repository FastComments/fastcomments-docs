By default, FastComments comes with trainable spam detection.

デフォルトで、FastComments には学習可能なスパム検出機能が付属しています。

As you moderate comments, and mark them as **Spam**, or mark comments automatically found as **Spam** as **Not Spam**, the spam
detection system will learn from these actions to more accurately determine what you want to be spam.

コメントをモデレートして **Spam** とマークしたり、自動的に **Spam** と判定されたコメントを **Not Spam** とマークしたりすると、スパム検出システムはこれらの操作から学習し、どのような投稿をスパムと見なしたいかをより正確に判定できるようになります。

Comments marked as **Spam** will not be automatically approved, so they will not show until explicitly marked as **Not Spam**.

**Spam** とマークされたコメントは自動的に承認されないため、明示的に **Not Spam** とマークされるまでは表示されません。

Spam Detection can be disabled via the Comment Moderation Settings page.

スパム検出はコメントモデレーション設定ページで無効にできます。

### Different Spam Detectors

### スパム検出の種類

FastComments supports three ways of detecting spam:

FastComments はスパムを検出する次の3つの方法をサポートします：

1. A traditional Naïve-Bayes classifier that is continuously trained, which is shared across all FastComments.com tenants.
1. 継続的に学習され、すべての FastComments.com テナントで共有される従来型の Naïve-Bayes 分類器。

2. A traditional Naïve-Bayes classifier that is continuously trained, which is **isolated** to your tenant.
2. 継続的に学習され、あなたのテナントに**隔離**された従来型の Naïve-Bayes 分類器。

3. Using ChatGPT 4.
3. ChatGPT 4 を使用する方法。

Everyone has access to the shared and isolated Naïve-Bayes classifiers.

すべてのユーザーは共有および隔離された Naïve-Bayes 分類器にアクセスできます。

The ChatGPT 4 option is selectable in the Comment Moderation Settings page if you are on Flex billing, since it bills based
on tokens used.

ChatGPT 4 オプションは、トークン使用量に基づいて課金されるため、Flex billing をご利用の場合にコメントモデレーション設定ページで選択できます。

### Trust Factor

### 信頼度

FastComments adjusts the spam filter for a user based on how much they are trusted for the given site.

FastComments は、そのサイトにおけるユーザーの信頼度に基づいてスパムフィルターを調整します。

For example, if administrators have pinned lots of their comments, then probably they are a very trustworthy user. Or, if
they have been a member of the site for a long time and have a lot of comments, then their trust factor may be high as well.

例えば、管理者がそのユーザーのコメントを多数ピン留めしている場合、そのユーザーは非常に信頼できると判断される可能性が高いです。また、長期間そのサイトのメンバーで多くのコメントを投稿している場合も、信頼度が高くなる傾向があります。

### SSO

### SSO

Comments posted by SSO users can be considered spam, and will be checked as such. The exception is if the SSO user
has the same email as a tenant user who has one or more of the following permissions:

SSO ユーザーが投稿したコメントはスパムと見なされ、スパムチェックの対象になります。例外は、SSO ユーザーのメールアドレスが、以下のいずれかの権限を持つテナントユーザーと同じである場合です：

- Account Owner
- Super Admin
- Comment Moderator Admin

SSO users with these permissions will not have their comments checked for spam.

これらの権限を持つ SSO ユーザーのコメントはスパムチェックされません。

### Repeated Messages

### 重複メッセージ

FastComments will detect and prevent repeated messages. It will also detect repeated message that are very similar to help prevent spam. This cannot
be disabled as it prevents our platform from being used for abuse. If you have a high trust factor, this is taken into account when doing repeated message prevention.

FastComments は重複するメッセージを検出してブロックします。スパム防止のため、非常に類似した繰り返しメッセージも検出します。この機能はプラットフォームの悪用を防ぐため無効にすることはできません。信頼度が高い場合は、重複メッセージの防止処理においてその点が考慮されます。