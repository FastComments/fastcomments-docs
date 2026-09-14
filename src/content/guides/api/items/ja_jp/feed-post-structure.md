A `FeedPost` オブジェクトは FastComments フィード内の投稿を表します。フィードは、各投稿が独自のコメントスレッドを持つ投稿のストリームで、Feed ウィジェットによってレンダリングされます。各投稿には作者、オプションのリッチコンテンツ、メディア、リンクがあり、タグ付けできるためフィードをフィルタリングできます。

`FeedPost` オブジェクトの構造は以下の通りです:

[inline-code-attrs-start title = 'FeedPost の構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPost {
    /** READONLY **/
    _id: string
    /** READONLY **/
    tenantId: string
    title?: string
    /** 投稿を作成した FastComments または SSO ユーザーの ID。 **/
    fromUserId?: string
    /** 設定されていない場合、ユーザーから取得されます。 **/
    fromUserDisplayName?: string | null
    /** READONLY。ユーザーから取得されます。 **/
    fromUserAvatar?: string | null
    /** フィードをフィルタリングするために使用されます。 **/
    tags?: string[]
    /** フィード内でのソート重み。値が大きいほど先に表示されます。 **/
    weight?: number
    /** 独自に使用できる自由形式のキー/バリューのペア。 **/
    meta?: Record<string, string>
    /** サニタイズされた HTML。 **/
    contentHTML?: string
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    /** READONLY **/
    createdAt: string
    /** READONLY。カウント対象のリアクションタイプ。 **/
    reacts?: Record<string, number>
    /** READONLY **/
    commentCount?: number | null
}

interface FeedPostMediaItem {
    title?: string
    /** クリック時にメディア項目がリンクする先。 **/
    linkUrl?: string
    /** 各レンダリングごとに1つのエントリ。ウィジェットが最適なものを選択します。 **/
    sizes: FeedPostMediaItemAsset[]
}

interface FeedPostMediaItemAsset {
    w: number
    h: number
    src: string
}

interface FeedPostLink {
    /** リンクテキスト（例: "今すぐサインアップ"）。 **/
    text?: string
    /** リンクと共に表示される見出し。 **/
    title?: string
    /** リンクと共に表示される説明。 **/
    description?: string
    url?: string
}
[inline-code-end]

注記:

- これらのフィールドの一部は `READONLY` とマークされています。これは API から返されますが、設定できません。
- 投稿のコメントは、`urlId` が `post:` に続いて投稿の `_id` となる通常のコメントです。その値を Comment API に使用して、投稿上のコメントを取得または作成できます。