---
Now we're going to generate your custom FastComments code. Use the wizard below to configure how you want FastComments to work on your GoHighLevel site:

[snippet id="gohighlevel-wizard"]

### Различни типове полета за коментари

You can configure the `TYPE = 'commenting'` line to switch the product used (for example you can change it to `live` for streaming chat or `collab` for collab chat).

### Поставяне на полето за коментари там, където искате

Let's say you want to put comment boxes on specific parts of the page and not the default locations.
Change this line:

    const TARGET_ELEMENT_ID = ''; // задайте, за да използвате режим на целеви div

To:

    const TARGET_ELEMENT_ID = 'fc_box'; // задайте, за да използвате режим на целеви div

Then in the GHL editor, click the "code" button and add where you want the comments to go:

[inline-code-attrs-start title = 'FastComments Div за GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Различен тип поле за коментари на страница

Let's say you want users to highlight and discuss pieces of text, or use the streaming chat UI instead.

First follow the steps above in "Putting The Comment Box Where You Want".

Note in that small snippet there's `type="commenting"`.

If you want to enable collab chat for example change type to `type="collab"`.

### Показвайте само на конкретни страници

If you don't set don't set `TARGET_ELEMENT_ID`, you can instead configure the `VALID_PATTERNS` variable, to set which URL routes the comments should show. By default, it will show
on pages that contain `/post` in the URL.

### Конфигуриране на Collab Chat

You can tell collab chat to only add collaborative functionality around HTML inside a specific area, for example, let's say you
add the footer code above and then add this div in the post/page content to enable collab chat:

[inline-code-attrs-start title = 'Collab Chat със зададено съдържание'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Then the paragraph element inside the `<div>` will have collab chat enabled, and nothing else on the page. If you don't
put any content in the `<div>` then it will enable collab chat on the entire post body.

---