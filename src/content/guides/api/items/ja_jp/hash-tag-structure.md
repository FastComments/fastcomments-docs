`HashTag` オブジェクトは、ユーザーが残せるタグを表します。HashTagsは、外部コンテンツへのリンクや、関連するコメントを結びつけるために使用できます。

`HashTag` オブジェクトの構造は以下の通りです:

[inline-code-attrs-start title = 'HashTag 構造'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface HashTag {
    /** Should start with the "#" or desired character. **/
    tag: string
    /** An optional URL that the hashtag can point to. Instead of filtering comments by hashtag, the UI will redirect to this upon click. **/
    url?: string
    /** READONLY **/
    createdAt: string
}
[inline-code-end]

注意:

- 一部のAPIエンドポイントでは、ハッシュタグがURL内で使用されます。値はURIエンコードすることを忘れないでください。例えば、`#` は `%23` として表現する必要があります。
- 一部のフィールドは `READONLY` とマークされています - これらはAPIから返されますが、設定することはできません。