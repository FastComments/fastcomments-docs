### Dark Mode Support

Image Chat includes built-in dark mode support. When you set `hasDarkBackground: true` in your configuration, the chat windows and UI elements automatically adjust to work well on dark backgrounds.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

The dark mode styling applies to chat windows, marker squares, and all interactive elements. If your site has a dark mode toggle, you can reinitialize the widget when the mode changes, or use the body class approach described below.

### Dynamic Dark Mode

If your site's dark mode is controlled by adding a `.dark` class to the body element, the Image Chat UI will automatically respect this without requiring reinitialization. The widget's styles are designed to respond to the presence of this class.

```css
/* Your dark mode CSS */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Custom Styling with CSS

You can customize the appearance of markers, chat windows, and other elements using CSS. The widget adds specific classes that you can target in your stylesheet.

The chat squares and windows use the FastComments comment bubble styling system, so any customizations you've applied to the standard commenting widget will also affect Image Chat.

### Chat Square Sizing

The `chatSquarePercentage` option controls the size of the clickable markers. The default is 5% of the image width:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Larger, more visible squares
});
```

Smaller values create more subtle markers that blend into the image. Larger values make markers more prominent and easier to click, especially on mobile devices or for accessibility purposes.

### Mobile Behavior

On screens under 768px wide, Image Chat automatically switches to a mobile-optimized layout. Chat windows appear fullscreen instead of floating next to the markers, providing better usability on small screens.

The markers remain visible at their responsive positions on the image. Users can tap any marker to open the fullscreen chat interface. This behavior is built-in and doesn't require any configuration.

### Chat Window Appearance

Chat windows are 300px wide on desktop with a 16px arrow pointing to the marker. The windows position themselves automatically based on available viewport space, using positioning classes like `to-right`, `to-left`, `to-top`, and `to-bottom`.

You can add custom CSS to adjust colors, fonts, spacing, or other visual properties of these windows. The chat windows use the same component structure as the standard FastComments widget, so they inherit any global customizations you've applied.

### Lazy Initialization

Chat windows initialize on hover for desktop users or immediately when created. This reduces the initial load overhead by only rendering the chat interface when users actually interact with a marker.

The lazy initialization happens transparently. Users don't notice any delay, but the browser doesn't need to render dozens of hidden chat windows if you have many markers on an image.

### Localization

Image Chat supports all the same localization options as the standard FastComments widget. Set the `locale` option to display UI text in different languages:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // French
});
```

FastComments supports dozens of languages. The locale setting affects all UI text including prompts, buttons, and placeholder text.

### Inherited Customization Options

Since Image Chat extends the standard commenting widget, it inherits all customization options from the base widget. This includes custom CSS classes, custom translations, avatar customization, date formatting, and much more.

See the main FastComments customization documentation for the complete list of customization options available.

### Working with Custom Fonts

If your site uses custom fonts, the Image Chat UI will inherit those fonts from your page's CSS. The chat windows render inside your page's DOM and respect your existing typography settings.

For best results, ensure your custom fonts are loaded before initializing Image Chat, or accept that there may be a brief flash of unstyled text while fonts load.

### Marker Visual Design

The square markers have a subtle visual design that makes them noticeable without overwhelming the image. You can customize their appearance with CSS if you want a different visual treatment.

The markers include hover states that provide feedback when users move their mouse over them. On touch devices, the tap interaction provides immediate feedback by opening the chat window.
