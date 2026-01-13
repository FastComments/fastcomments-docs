### How Text Selection Works

When users select text within the Collab Chat container, the widget captures that selection and allows them to start a discussion. The selection can be as small as a single word or as large as multiple paragraphs spanning different elements.

### Selection Delay

On desktop devices, there's a 3.5 second delay between when a user selects text and when the discussion prompt appears. This prevents the UI from flickering when users are simply highlighting text to copy or read. On mobile devices, the prompt appears immediately since text selection is more deliberate on touch screens.


### Unique Conversation IDs

Each conversation gets a unique `urlId` that combines the page URL, the DOM element path, and the serialized text range. This ensures that each text selection creates a distinct conversation that can be found again later.

If you provide a custom `urlId` in your configuration, it will be combined with the element path and text range to create the final identifier.

### Visual Highlights

When a discussion exists for a particular text selection, that text receives a visual highlight. The highlight is implemented using background colors and appears on hover or when the associated chat window is open.

The highlighting system works by wrapping the selected text in a special element that can be styled. This approach ensures that highlights remain accurate even when the underlying HTML structure is complex.

### Chat Window Positioning

When a user clicks on a highlight or creates a new annotation, a chat window appears near the selected text. The widget automatically calculates the best position for this window based on available viewport space.

The positioning system uses CSS classes like `to-right`, `to-left`, `to-top`, and `to-bottom` to indicate which direction the chat window should extend from the highlight. On mobile devices (screens under 768px wide), the chat window always appears fullscreen for better usability.

### Chat Window Dimensions

Chat windows are 410px wide on desktop with 20px spacing and a 16px visual arrow pointing to the highlighted text. These dimensions are fixed to ensure consistent behavior, but you can customize the appearance with CSS.

### Cross-Element Selections

Users can select text that spans multiple HTML elements, such as highlighting from the middle of one paragraph through the start of another. The range serialization system handles this correctly and will highlight all the selected text even across element boundaries.

### Browser Compatibility

The text selection system uses the standard `window.getSelection()` API which is supported in all modern browsers. For older versions of Internet Explorer, it falls back to `document.selection` for compatibility.

### Selection Persistence

Once a conversation is created for a text selection, that annotation persists even if the page is reloaded. The serialized range and DOM path allow the widget to restore highlights in the exact same location when users return to the page.

This works reliably as long as your page content remains stable. If you change the text content or restructure your HTML, existing annotations may no longer align correctly with the text. For this reason, it's best to avoid major content changes on pages with active annotations, or consider migrating annotations when content changes are necessary.
