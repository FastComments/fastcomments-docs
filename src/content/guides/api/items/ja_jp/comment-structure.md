A `Comment` object represents a comment left by a user.

親コメントと子コメントの関係は `parentId` によって定義されます。

Comment オブジェクトの構造は次のとおりです:

[inline-code-attrs-start title = 'コメント構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface Comment {
    /** READONLY: スパムエンジンがコメントをスパムと判断した場合に true に設定されます。 **/
    aiDeterminedSpam?: boolean
    /** コメントが表示されるように承認されているかどうか。コメントの保存時に true に設定され、そうでなければ非表示になります。 **/
    approved?: boolean
    /** ユーザーのアバター。 **/
    avatarSrc?: string
    /** 子コメント。すべてのシナリオで埋められるわけではありません。API で asTree を true にした場合に使用されます。 **/
    children: Comment[]
    /** 投稿者が入力した生のコメント。 **/
    comment: string
    /** READONLY: 投稿者のコメントを HTML にパースしたもの。 **/
    commentHTML?: string
    /** 匿名コメントがオフの場合は必須の投稿者のメールアドレス。 **/
    commenterEmail?: string
    /** 投稿者のリンク（例えばブログ）。 **/
    commenterLink?: string
    /** 投稿者の名前。常に必須。利用できない場合は "Anonymous" などに設定してください。 **/
    commenterName: string
    /** コメントが残された日時（UTC エポック）。 **/
    date: number
    /** コメントの「表示ラベル」— 例えば "Admin"、"Moderator"、あるいは "VIP User" のようなもの。 **/
    displayLabel?: string
    /** コメントが投稿されたドメイン。 **/
    domain?: string
    /** READONLY: コメントがフラグ付けされた回数。 **/
    flagCount?: number
    /** コメント内で正常に解析された #ハッシュタグ。検索のためにハッシュタグを手動で追加することもできますが、自動的にコメントテキスト内に表示はされません。 **/
    hashTags?: CommentHashTag[]
    /** READONLY: コメントに画像が含まれているかどうか。 **/
    hasImages?: boolean
    /** READONLY: コメントにリンクが含まれているかどうか。 **/
    hasLinks?: boolean
    /** READONLY: ユニークなコメント ID。 **/
    id: string
    /** 作成時のみ！保存用にハッシュ化されます。 **/
    ip?: string
    /** READONLY: 現在のユーザーがこのコメントを書いたユーザーをブロックしているかどうか。 **/
    isBlocked?: boolean
    /** READONLY: 管理者によるコメントかどうか。userId に基づいて自動設定されます。 **/
    isByAdmin?: boolean
    /** READONLY: モデレーターによるコメントかどうか。userId に基づいて自動設定されます。 **/
    isByModerator?: boolean
    /** 他の設定のためにプレースホルダを残す必要があった場合など、コメントがソフト削除された場合は true に設定されます。 **/
    isDeleted?: boolean
    /** ユーザーのアカウントが削除され、コメントを保持する必要があった場合は true に設定されます。 **/
    isDeletedUser?: boolean
    /** READONLY: 現在ログイン中のユーザー（contextUserId）がフラグを付けているかどうか。 **/
    isFlagged?: boolean
    /** コメントがピン留めされているかどうか。 **/
    isPinned?: boolean
    /** コメントがロックされているかどうか。true の場合、ロック解除されるまで誰も（モデレーターを含む）返信、編集、削除ができません。 **/
    isLocked?: boolean
    /** コメントがスパムかどうか。 **/
    isSpam?: boolean
    /** READONLY: 現在のユーザー（contextUserId）に対してコメントがダウンボートされているかどうか。 **/
    isVotedDown?: boolean
    /** READONLY: 現在のユーザー（contextUserId）に対してコメントがアップボートされているかどうか。 **/
    isVotedUp?: boolean
    /** コメントのロケール。指定がない場合は Accept-Language ヘッダから導出されます。 **/
    locale?: 'de_de' | 'en_us' | 'es_es' | 'fr_fr' | 'it_it' | 'ja_jp' | 'ko_kr' | 'pl_pl' | 'pt_br' | 'ru_ru' | 'tr_tr' | 'zh_cn' | 'zh_tw'
    /** READONLY: コメント内で正常に解析された @メンション。 **/
    mentions?: CommentUserMention[]
    /** コメントに関連付けられたオプションのメタデータ。 **/
    meta?: Record<string, string | number | boolean>
    /** このコメントに関連付けられたモデレーショングループ ID のオプションリスト。 **/
    moderationGroupIds?: string[]|null
    /** READONLY: 現在のユーザー（contextUserId）がこのコメントに対して行った投票に対応する vote オブジェクトの ID。 **/
    myVoteId?: string
    /** コメントに対して投稿者向けの通知が送信されたかどうか。インポート時に通知を送信しないようにするにはこれを true に設定します。 **/
    notificationSentForParent?: boolean
    /** テナントユーザー向けの通知がこのコメントに対して送信されたかどうか。インポート時に通知を送信しないようにするにはこれを true に設定します。 **/
    notificationSentForParentTenant?: boolean
    /** このコメントがあったページのタイトル。 **/
    pageTitle?: string
    /** 返信の場合、返信先のコメントの ID。 **/
    parentId?: string|null
    /** コメントがレビュー済みとしてマークされているかどうか。 **/
    reviewed: boolean
    /** コメントが属するテナントの ID。 **/
    tenantId: string
    /** コメントを書いたユーザー。名前/メールでコメントを保存すると自動的に作成されます。 **/
    userId?: string|null
    /** コメントが表示される場所（ブログ記事など）への URL。 **/
    url: string
    /** 渡した urlId を「正規化」した値。保存時にこのフィールドを指定しますが、コメントを取得するとこの値が「クリーン化」され、元の値は "urlIdRaw" に移動します。 **/
    urlId: string
    /** READONLY: あなたが渡した元の urlId。 **/
    urlIdRaw?: string
    /** ユーザーとこのコメントが検証済みかどうか。 **/
    verified: boolean
    /** 賛成票の数。 **/
    votesUp?: number
    /** 反対票の数。 **/
    votesDown?: number
    /** コメントの「カーマ」（= votes up - votes down）。 **/
    votes?: number
}
[inline-code-end]

これらのフィールドの一部には `READONLY` と記載されています — これらは API によって返されますが設定することはできません。

### コメントテキスト構造

コメントは FastComments 独自のマークダウンで記述されます。これは通常のマークダウンに加えて、画像用の従来の `bbcode` スタイルタグ（例: `[img]path[/img]`）が使えるというものです。

テキストは 2 つのフィールドに保存されます。ユーザーが入力したテキストは変更されずに `comment` フィールドに保存されます。これはレンダリングされ `commentHTML` フィールドに保存されます。

許可されている HTML タグは `b, u, i, strike, pre, span, code, img, a, strong, ul, ol, li, and br` です。

HTML をレンダリングすることを推奨します。許可される HTML は非常に限定的なサブセットなので、レンダラーを作るのは比較的簡単です。たとえば React Native や Flutter 向けのライブラリが複数存在し、この作業を助けてくれます。

`comment` フィールドの正規化されていない値をレンダリングすることもできます。 [例のパーサーはこちら。](https://github.com/FastComments/fastcomments-code-examples/blob/master/custom-client/client/parse-comment.js)

その例のパーサーは HTML に対応するように調整し、HTML タグをプラットフォームでレンダリングするために期待される要素へ変換することもできます。

### タグ付け

ユーザーがコメント内でタグ付けされると、その情報は `mentions` と呼ばれるリストに格納されます。そのリスト内の各オブジェクトは次の構造を持ちます。

[inline-code-attrs-start title = 'コメントのメンションオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentUserMention {
    /** ユーザー id。SSO ユーザーの場合はあなたのテナント id が接頭辞として付与されます。 **/
    id: string
    /** 最終的な @メンションタグのテキスト（@ 記号を含む）。 **/
    tag: string
    /** 元の @メンションタグのテキスト（@ 記号を含む）。 **/
    rawTag: string
    /** タグ付けされたユーザーのタイプ。user = FastComments.com アカウント、sso = SSOUser。 **/
    type: 'user'|'sso'
    /** ユーザーが通知をオプトアウトしている場合でも、これが true に設定されます。 **/
    sent: boolean
}
[inline-code-end]

### ハッシュタグ

ハッシュタグが使用されて正常に解析された場合、その情報は `hashTags` と呼ばれるリストに格納されます。そのリスト内の各オブジェクトは次の構造を持ちます。ハッシュタグは、`retain` が設定されている場合、コメントテキストに存在しなくてもクエリ用にコメントの `hashTags` 配列に手動で追加することができます。

[inline-code-attrs-start title = 'コメントのハッシュタグオブジェクト'; type = 'typescript'; inline-code-attrs-end]
[inline-code-start]
interface CommentHashTag {
    /** ハッシュタグの id。 **/
    id: string
    /** 最終的な #ハッシュタグのテキスト（# 記号を含む）。 **/
    tag: string
    /** ハッシュタグがカスタム URL に関連付けられている場合は定義されます。 **/
    url?: string
    /** コメント更新時にコメントテキストに存在しなくてもハッシュタグを保持するかどうか。コメントテキストを変更せずにコメントにタグ付けするのに便利です。 **/
    retain?: boolean
}
[inline-code-end]

---