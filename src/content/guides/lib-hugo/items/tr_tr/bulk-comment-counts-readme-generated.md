Birçok sayfanın yanında aynı anda bir yorum sayısı göstermek için (bir blog indeksi, bir bölüm listesi), toplu sayım widget'ını kullanın. Bu, sayfadaki tüm sayımları tek bir istekte getirir. İki parçadan oluşur: her öğenin yanındaki bir işaretleyici ve listenin ardından yapılan bir init çağrısı.

Bir liste şablonunda (`layouts/_default/list.html`):

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

`count-marker.html` şu öğeyi render eder: `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>`, yorumlar widget'ının o sayfa için kullandığı aynı tanımlayıcıyı (sayfanın `urlId`'si veya `urlId` yoksa permalink'i) kullanır, böylece sayımlar gerçek konu başlıklarıyla hizalanır. `bulk-count.html` ise bunları dolduran tek isteği gönderir.

Eğer işaretleyicileri kendiniz yazıyorsanız (örneğin bir sayfanın Markdown'unda), init çağrısını oluşturmak için kısa kodu kullanın:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```