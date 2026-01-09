A `Comment` オブジェクトは、ユーザーが残したコメントを表します。

親コメントと子コメントの関係は `parentId` を通じて定義されます。

Comment オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'コメント構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** 読み取り専用: スパムエンジンがコメントをスパムと判定した場合に true に設定されます。 **/
    aiDeterminedSpam?: boolean
    /** コメントが表示承認されているか。コメント保存時に true に設定されます。そうでない場合は非表示になります。 **/
    approved?: boolean
    /** ユーザーのアバター。 **/
    avatarSrc?: string
    /** 子コメント。すべてのシナリオで設定されるわけではありません。API で asTree を true にした場合に使用されます。 **/
    children: Comment[]
    /** コメント投稿者の生のコメント。 **/
    comment: string
    /** 読み取り専用: コメント投稿者のコメントを HTML に変換したもの。 **/
    commentHTML?: string
    /** コメント投稿者のメールアドレス。匿名コメントが無効の場合は必須です。 **/
    commenterEmail?: string
    /** コメント投稿者のリンク（例: ブログ）。 **/
    commenterLink?: string
    /** コメント投稿者の名前。常に必須です。利用できない場合は「Anonymous」などに設定してください。 **/
    commenterName: string
    /** コメントが残された日付（UTC エポック）。 **/
    date: number
    /** コメントの「表示ラベル」— 例えば「Admin」「Moderator」や「VIP User」など。 **/
    displayLabel?: string
    /** コメントが投稿されたドメイン。 **/
    domain?: string
    /** 読み取り専用: コメントが通報された回数。 **/
    flagCount?: number
    /** コメント内で成功裏に解析された #ハッシュタグ。クエリのためにハッシュタグを手動で追加することもできますが、コメント本文には自動的には表示されません。 **/
    hashTags?: CommentHashTag[]
    /** 読み取り専用: コメントに画像が含まれているか。 **/
    hasImages?: boolean
    /** 読み取り専用: コメントにリンクが含まれているか。 **/
    hasLinks?: boolean
    /** 読み取り専用: ユニークなコメント ID。 **/
    id: string
    /** 作成時のみ！保存のためにハッシュされます。 **/
    ip?: string
    /** 読み取り専用: 現在のユーザーがこのコメントを書いたユーザーをブロックしているか（contextUserId）。 **/
    isBlocked?: boolean
    /** 読み取り専用: コメントが管理者によるものか。userId に基づいて自動設定されます。 **/
    isByAdmin?: boolean
    /** 読み取り専用: コメントがモデレーターによるものか。userId に基づいて自動設定されます。 **/
    isByModerator?: boolean
    /** コメントがソフト削除された場合（何らかの別の設定のためプレースホルダを残す必要があった場合）に true に設定されます。 **/
    isDeleted?: boolean
    /** ユーザーのアカウントが削除され、コメントを保持する必要があった場合に true に設定されます。 **/
    isDeletedUser?: boolean
    /** 読み取り専用: 現在ログインしているユーザー（contextUserId）によって通報されたか。 **/
    isFlagged?: boolean
    /** コメントがピン留めされているか。 **/
    isPinned?: boolean
    /** 新しい返信ができないようにロックされているか（モデレーターは引き続き返信可能）。 **/
    isLocked?: boolean
    /** コメントがスパムかどうか。 **/
    isSpam?: boolean
    /** 読み取り専用: 現在のユーザー（contextUserId）によるダウン投票か。 **/
    isVotedDown?: boolean
    /** 読み取り専用: 現在のユーザー（contextUserId）によるアップ投票か。 **/
    isVotedUp?: boolean
    /** コメントの言語ロケール。指定がない場合は HTTP の言語受け入れヘッダから決定されます。 **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** 読み取り専用: コメント内で成功裏に解析された @メンション。 **/
    mentions?: CommentUserMention[]
    /** コメントに関連付けられたオプションのメタデータ。 **/
    meta?: Record<string, string | number | boolean>
    /** このコメントに関連付けられたモデレーショングループ ID のオプションリスト。 **/
    moderationGroupIds?: string[]|null
    /** 読み取り専用: このコメントに対して現在のユーザー（contextUserId）が行った投票に対応する vote オブジェクトの ID。 **/
    myVoteId?: string
    /** このコメントに対して返信者向けの通知が送信されたか。インポート時に通知を送らないようにするには、これを true に設定します。 **/
    notificationSentForParent?: boolean
    /** テナントユーザー向けの通知が送信されたか。インポート時に通知を送らないようにするには、これを true に設定します。 **/
    notificationSentForParentTenant?: boolean
    /** このコメントがあったページのタイトル。 **/
    pageTitle?: string
    /** 返信している場合、返信先のコメント ID。 **/
    parentId?: string|null
    /** コメントがレビュー済みとしてマークされているか。 **/
    reviewed: boolean
    /** コメントが属するテナント ID。 **/
    tenantId: string
    /** コメントを書いたユーザー。名前/メールでコメントを保存すると自動的に作成されます。 **/
    userId?: string|null
    /** このコメントが表示される場所（例: ブログ記事）の URL。 **/
    url: string
    /** あなたが渡した urlId の「クリーン」なバージョン。保存時にはこのフィールドを指定しますが、コメントを取得するときには「クリーン」化され、元の値は "urlIdRaw" に移されます。 **/
    urlId: string
    /** 読み取り専用: あなたが渡した元の urlId。 **/
    urlIdRaw?: string
    /** ユーザーとこのコメントが検証済みか。 **/
    verified: boolean
    /** 賛成票の数。 **/
    votesUp?: number
    /** 反対票の数。 **/
    votesDown?: number
    /** コメントの「カルマ」（= 賛成票 - 反対票）。 **/
    votes?: number
}
[inline-code-end]

これらのフィールドの一部は `READONLY` としてマークされています — これらは API によって返されますが、設定することはできません。

### コメントテキストの構造

コメントは FastComments 仕様のマークダウンで記述されます。これはマークダウンに、画像用の従来の bbcode スタイルタグ（`[img]path[/img]` のような）を追加したものです。

テキストは 2 つのフィールドに保存されます。ユーザーが入力したテキストは修正されず `comment` フィールドに保存されます。これがレンダリングされ、`commentHTML` フィールドに保存されます。

許可されている HTML タグは `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br` です。

HTML は非常に限定されたサブセットなので、HTML をレンダリングすることを推奨します。レンダラーを作るのは比較的簡単です。例えば React Native や Flutter 用のライブラリがいくつかあります。

`comment` フィールドの正規化されていない値をレンダリングすることも選択できます。 [例のパーサーはこちら。](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js)

この例のパーサーは HTML に合わせて調整し、HTML タグをプラットフォームでレンダリングするための期待される要素に変換することもできます。

### タギング

ユーザーがコメント内でタグ付けされた場合、その情報は `mentions` と呼ばれるリストに保存されます。リスト内の各オブジェクトは次の構造を持ちます。

[inline-code-attrs-start title = 'コメントのメンションオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ユーザー ID。SSO ユーザーの場合はテナント ID がプレフィックスとして付与されます。 **/
    id: string
    /** 最終的な @メンションのタグテキスト（@ 記号を含む）。 **/
    tag: string
    /** 元の @メンションのタグテキスト（@ 記号を含む）。 **/
    rawTag: string
    /** タグ付けされたユーザーのタイプ。user = FastComments.com アカウント、sso = SSOUser。 **/
    type: 'user'|'sso'
    /** ユーザーが通知をオプトアウトしている場合でも、これは true に設定されます。 **/
    sent: boolean
}
[inline-code-end]

### ハッシュタグ

ハッシュタグが使用され、正常に解析された場合、その情報は `hashTags` と呼ばれるリストに保存されます。リスト内の各オブジェクトは次の構造を持ちます。`retain` が設定されている場合、ハッシュタグはクエリ用にコメントの `hashTags` 配列に手動で追加することもできます。

[inline-code-attrs-start title = 'コメントのハッシュタグオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** ハッシュタグ ID。 **/
    id: string
    /** 最終的な #ハッシュタグのタグテキスト（# 記号を含む）。 **/
    tag: string
    /** ハッシュタグがカスタム URL に関連付けられている場合、ここに定義されます。 **/
    url?: string
    /** コメントが更新されたときに、コメント本文に存在しない場合でもハッシュタグを保持するか。コメントテキストを変更せずにコメントにタグ付けする場合に便利です。 **/
    retain?: boolean
}
[inline-code-end]