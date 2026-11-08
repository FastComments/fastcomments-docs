एक साथ कई पेजों के बगल में कमेंट काउंट दिखाने के लिए (एक ब्लॉग इंडेक्स, एक सेक्शन लिस्ट), बल्क काउंट विजेट का उपयोग करें। यह पेज पर मौजूद हर काउंट को एक ही रिक्वेस्ट में फ़ेच करता है। इसके दो हिस्से हैं: प्रत्येक आइटम के बगल में एक मार्कर, और लिस्ट के बाद एक init कॉल।

एक लिस्ट टेम्पलेट में (`layouts/_default/list.html`):

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

`count-marker.html` `<span class="fast-comments-count" data-fast-comments-url-id="..."></span>` रेंडर करता है, उसी आइडेंटिफ़ायर का उपयोग करते हुए जो उस पेज के लिए कमेंट्स विजेट उपयोग करता है (उसका `urlId`, या जब कोई `urlId` सेट न हो तो उसका परमालिंक), ताकि काउंट असली थ्रेड्स से मेल खाएं। `bulk-count.html` वह एकल रिक्वेस्ट जारी करता है जो इन्हें भरती है।

यदि आप मार्कर स्वयं लिखते हैं (उदाहरण के लिए किसी पेज के Markdown में), तो init कॉल जारी करने के लिए शॉर्टकोड का उपयोग करें:

```text
<span class="fast-comments-count" data-fast-comments-url-id="article-1"></span>
<span class="fast-comments-count" data-fast-comments-url-id="article-2"></span>

\{{< fastcomments-comment-count-bulk >}}
```