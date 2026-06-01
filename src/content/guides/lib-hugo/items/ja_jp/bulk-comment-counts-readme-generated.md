複数のページ（ブログの一覧やセクションの一覧など）にコメント数を同時に表示するには、bulk count ウィジェットを使用します。ページ上のすべてのカウントを単一のリクエストで取得します。構成は2つに分かれます：各項目の横に置くマーカーと、一覧の後に行う1回の初期化（init）呼び出しです。

リストテンプレート（`layouts/_default/list.html`）内では:

```go-html-template
<ul>
  \{{ range .Pages }}
    <li>
      <a href="\{{ .RelPermalink }}">\{{ .Title }}</a>
      \{{ partial "fastcomments/count-marker.html" . }}
    </li>
  \{{ end }}
</ul>
\{{ partial "fastcomments/bulk-count.html" (dict "page" .) }}
```

`count-marker.html` は `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>` をレンダリングします。これは、そのページのコメントウィジェットが使用するのと同じ識別子（`urlId`、`urlId` が設定されていない場合はそのパーマリンク）を使用するため、カウントが実際のスレッドと一致します。`bulk-count.html` はそれらを埋める単一のリクエストを送信します。

マーカーを自分で書く場合（例えばページの Markdown 内で）、代わりにショートコードを使用して初期化呼び出しを出力してください:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```