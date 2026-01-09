A `HashTag` オブジェクトはユーザーが残すことのできるタグを表します。ハッシュタグは外部のコンテンツへのリンクに使用したり、関連するコメントを結びつけるために使用できます。

The structure for the `HashTag` object is as follows:

[inline-code-attrs-start title = 'HashTag 構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** "#" または任意の指定文字で始める必要があります。 **/
    tag: string
    /** ハッシュタグが指すオプションのURL。ハッシュタグでコメントをフィルタリングする代わりに、UIはクリック時にこのURLへリダイレクトします。 **/
    url?: string
    /** 読み取り専用 **/
    createdAt: string
}
[inline-code-end]

Notes:

- In some API endpoints you will see that the hashtag is used in the URL. Remember to URI-Encoded values. For example, `#` should instead be represented as `%23`.
- Some of these fields are marked `READONLY` - these are returned by the API but cannot be set.