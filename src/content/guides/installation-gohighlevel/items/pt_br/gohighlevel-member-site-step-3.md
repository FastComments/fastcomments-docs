---
Now we're going to generate your custom FastComments code. Use the wizard below to configure how you want FastComments to work on your GoHighLevel site:

[snippet id="gohighlevel-wizard"]

### Different Comment Box Types

You can configure the `TYPE = 'commenting'` line to switch the product used (for example you can change it to `live` for streaming chat or `collab` for collab chat).

### Putting The Comment Box Where You Want

Let's say you want to put comment boxes on specific parts of the page and not the default locations.
Change this line:

    const TARGET_ELEMENT_ID = ''; // definir para usar o modo de div alvo

To:

    const TARGET_ELEMENT_ID = 'fc_box'; // definir para usar o modo de div alvo

Then in the GHL editor, click the "code" button and add where you want the comments to go:

[inline-code-attrs-start title = 'Div do FastComments no GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Different Comment Box Type Per-Page

Let's say you want users to highlight and discuss pieces of text, or use the streaming chat UI instead.

First follow the steps above in "Putting The Comment Box Where You Want".

Note in that small snippet there's `type="commenting"`.

If you want to enable collab chat for example change type to `type="collab"`.

### Only Show On Specific Pages

If you don't set `TARGET_ELEMENT_ID`, you can instead configure the `VALID_PATTERNS` variable, to set which URL routes the comments should show. By default, it will show
on pages that contain `/post` in the URL.

### Configuring Collab Chat

You can tell collab chat to only add collaborative functionality around HTML inside a specific area, for example, let's say you
add the footer code above and then add this div in the post/page content to enable collab chat:

[inline-code-attrs-start title = 'Collab Chat com Conteúdo Especificado'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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