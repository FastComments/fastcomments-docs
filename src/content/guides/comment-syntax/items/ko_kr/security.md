There are multiple aspects to security when letting people add content to a website
and then rendering that content on many different types of devices.

### Preventing Formatting Abuse

사람들이 콘텐츠를 작성할 때 의도적으로 시각적으로 산만하게 만들거나 텍스트 서식을 악용하여 토론의 가치를 떨어뜨리는 경우가 있습니다.

FastComments는 서식과 관련된 악용을 방지하기 위해 여러 조치를 취합니다:

- Large repeated consecutive newlines are collapsed.
- We don't render headings (they become normal text).
- We don't allow CSS or custom colors.

### Preventing Exploits

HTML을 렌더링하는 시스템에서는 악용(익스플로잇)이 발생할 수 있습니다. FastComments는 이를 방지하기 위해 여러 조치를 취합니다:

- We only allow an explicitly defined set of HTML tags.
- We only allow an explicitly defined set of HTML tag attributes.
- We purify and sanitize all inputs.
  - This is done via the [DOMPurify](https://www.npmjs.com/package/dompurify) and [sanitizeHtml](https://www.npmjs.com/package/sanitize-html) libraries.
  - We chose these libraries as being well tested (having over 4 and 1 million downloads per week, respectively).

This means that users can't do things like write `<script>` or `<style>` tags, or try to add `onload=alert()` type scripts to images or other content.

The HTML tags we allow are as follows:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`

The `<iframe>` tag is not allowed by default. If you turn on Allow Media Embeds, iframes are also permitted, but only when their source is one of a built-in list of trusted providers (such as YouTube, Vimeo, SoundCloud, and Spotify) or a hostname you have explicitly added. Iframes from any other source are removed.