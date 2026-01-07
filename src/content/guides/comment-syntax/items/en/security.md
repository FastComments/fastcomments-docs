There are multiple aspects to security when letting people add content to a website
and then rendering that content on many different types of devices.

### Preventing Formatting Abuse

People can write content that is intentionally visually distracting
and detracts value from discussions by abusing text formatting.

FastComments does a number of things to prevent abuse in regard to formatting:

- Large repeated consecutive newlines are collapsed.
- We don't render headings (they become normal text).
- We don't allow CSS or custom colors.

### Preventing Exploits

Exploits can be created in systems that render HTML. FastComments does several things to prevent this:

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
