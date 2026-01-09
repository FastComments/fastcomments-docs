### Υποστήριξη Σκοτεινής Λειτουργίας

### Δυναμική Σκοτεινή Λειτουργία

If your site's dark mode is controlled by adding a `.dark` class to the body element, the Collab Chat UI will automatically respect this without requiring reinitialization. The widget's styles are designed to respond to the presence of this class.

[inline-code-attrs-start title = 'Παράδειγμα CSS σκοτεινής λειτουργίας'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Το CSS σκοτεινής λειτουργίας σας */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Προσαρμοσμένο στυλ με CSS

You can customize the appearance of highlights, chat windows, and other elements using CSS. The widget adds specific classes that you can target in your stylesheet.

Text highlights use the FastComments comment bubble styling system, so any customizations you've applied to the standard commenting widget will also affect Collab Chat.

### Προσαρμογή επάνω μπάρας

The top bar shows the number of users online and the number of discussions. You can customize its position by providing a custom element as the `topBarTarget`:

[inline-code-attrs-start title = 'Προσαρμοσμένη θέση επάνω μπάρας'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Or disable it entirely by setting it to `null`:

[inline-code-attrs-start title = 'Απενεργοποίηση επάνω μπάρας'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Συμπεριφορά σε κινητές συσκευές

On screens under 768px wide, Collab Chat automatically switches to a mobile-optimized layout. Chat windows appear fullscreen instead of floating next to the text, and the selection delay is removed for more immediate interaction.

This behavior is built-in and doesn't require any configuration. The widget detects screen size automatically and adjusts accordingly.

### Εμφάνιση παραθύρου συνομιλίας

Chat windows are 410px wide on desktop with a 16px arrow pointing to the highlighted text. The windows position themselves automatically based on available viewport space, using positioning classes like `to-right`, `to-left`, `to-top`, and `to-bottom`.

You can add custom CSS to adjust colors, fonts, spacing, or other visual properties of these windows. The chat windows use the same component structure as the standard FastComments widget, so they inherit any global customizations you've applied.

### Τοπικοποίηση

Collab Chat supports all the same localization options as the standard FastComments widget. Set the `locale` option to display UI text in different languages:

[inline-code-attrs-start title = 'Ορισμός locale'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Ισπανικά
});
[inline-code-end]

FastComments supports dozens of languages. The locale setting affects all UI text including prompts, buttons, and placeholder text.

### Κληρονομημένες επιλογές προσαρμογής

Since Collab Chat extends the standard commenting widget, it inherits all customization options from the base widget. This includes custom CSS classes, custom translations, avatar customization, date formatting, and much more.

See the main FastComments customization documentation for the complete list of customization options available.

### Χρήση προσαρμοσμένων γραμματοσειρών

If your site uses custom fonts, the Collab Chat UI will inherit those fonts from your page's CSS. You may have to create a widget customization rule and `@import` any fonts in the custom CSS in that rule if you
want the floating chat window to use the same fonts.