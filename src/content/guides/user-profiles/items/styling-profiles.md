When User Profiles are opened in the context of your site (via the comment widget), any custom CSS styles you've applied to your FastComments widget are automatically injected into the profile modal.

### How Profile Styling Works

When a user clicks on a profile link from your comment widget:

1. A profile modal/flyover opens
2. Your widget's custom CSS is automatically injected into the profile view
3. The profile inherits your site's comment styling
4. Users see a consistent visual experience

This ensures that User Profiles feel native to your site rather than appearing as a separate, differently-styled interface.

### Style Inheritance

**What Gets Injected:**
Any CSS you've added to customize your FastComments widget will automatically apply to profile modals opened from your site. This includes:

- Color schemes and themes
- Font families and sizes
- Border styles and spacing
- Button styles
- Background colors
- Custom class definitions
- Dark mode styles

**Automatic Injection:**
You don't need to do anything special - if you've customized your widget's appearance, those styles automatically carry over to profile views.

### Customizing Profile Styles

To style profiles, simply add CSS to your FastComments widget configuration:

#### Example: Basic Color Customization

```javascript
window.FastCommentsUI(document.getElementById('fastcomments-widget'), {
    tenantId: 'your-tenant-id',
    // ... other config
});
```

Then add custom CSS to your page:

```css
/* These styles will apply to both comments and profiles */
.fastcomments-widget {
    --fc-primary-color: #007bff;
    --fc-background-color: #ffffff;
    --fc-text-color: #333333;
}
```

#### Example: Dark Mode

If you've implemented dark mode for your comments:

```css
.dark-mode .fastcomments-widget {
    --fc-primary-color: #4a9eff;
    --fc-background-color: #1e1e1e;
    --fc-text-color: #e0e0e0;
}
```

The profile modal will automatically inherit these dark mode styles when opened.

### Profile-Specific Styling

While most styles are shared, you can target profile-specific elements:

```css
/* Target the profile header */
.fastcomments-profile-header {
    background: linear-gradient(to right, #667eea, #764ba2);
}

/* Style profile badges */
.fastcomments-profile-badges {
    padding: 1rem;
    border-radius: 8px;
}

/* Customize profile tabs */
.fastcomments-profile-tabs {
    border-bottom: 2px solid var(--fc-primary-color);
}
```

### Using CSS Variables

FastComments supports CSS custom properties (variables) for easy theming:

```css
.fastcomments-widget {
    /* Colors */
    --fc-primary-color: #your-color;
    --fc-secondary-color: #your-color;
    --fc-background-color: #your-color;
    --fc-text-color: #your-color;
    --fc-border-color: #your-color;

    /* Typography */
    --fc-font-family: 'Your Font', sans-serif;
    --fc-font-size: 14px;

    /* Spacing */
    --fc-padding: 1rem;
    --fc-margin: 0.5rem;

    /* Borders */
    --fc-border-radius: 4px;
    --fc-border-width: 1px;
}
```

These variables will be respected in both the comment widget and profile views.

### Responsive Design

Ensure your profile styles work across different screen sizes:

```css
/* Desktop styles */
.fastcomments-profile-header {
    height: 300px;
}

/* Tablet and mobile */
@media (max-width: 768px) {
    .fastcomments-profile-header {
        height: 200px;
    }
}
```

Profile modals are responsive by default, but custom styles should account for different viewport sizes.

### Best Practices

**Consistency:**
- Use the same color palette for comments and profiles
- Match typography across both interfaces
- Maintain consistent spacing and borders
- Ensure dark mode works in both contexts

**Performance:**
- Keep CSS concise and efficient
- Avoid overly specific selectors
- Use CSS variables for easy maintenance
- Minimize custom styles when possible

**Accessibility:**
- Maintain sufficient color contrast
- Ensure interactive elements are clearly visible
- Don't rely solely on color to convey information
- Test with screen readers if you add complex custom styling

**Testing:**
1. Test profile modals after adding new widget styles
2. Verify styles work in both light and dark modes
3. Check responsive behavior on mobile devices
4. Ensure all profile tabs render correctly with your styles

### Common Customizations

#### Matching Your Brand Colors

```css
.fastcomments-widget {
    --fc-primary-color: #ff6b6b;  /* Your brand color */
    --fc-background-color: #f8f9fa;
    --fc-border-color: #dee2e6;
}

.fastcomments-widget button {
    background-color: var(--fc-primary-color);
    border-radius: 6px;
    font-weight: 600;
}
```

#### Custom Fonts

```css
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap');

.fastcomments-widget {
    --fc-font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
}
```

#### Rounded Design

```css
.fastcomments-widget {
    --fc-border-radius: 12px;
}

.fastcomments-widget .avatar {
    border-radius: 50%;
}

.fastcomments-widget button,
.fastcomments-widget input {
    border-radius: 8px;
}
```

### Debugging Profile Styles

If your styles aren't appearing in profiles:

1. **Check CSS Specificity:** Ensure your selectors are specific enough
2. **Inspect the Profile Modal:** Use browser dev tools to see which styles are applied
3. **Verify CSS Loading:** Make sure your stylesheet is loaded before the widget
4. **Check for Conflicts:** Look for CSS that might be overriding your styles
5. **Clear Cache:** Browser cache can sometimes serve old styles

### Limitations

**What Doesn't Get Injected:**
- Page-level styles that don't target FastComments elements
- Styles from separate stylesheets not connected to the widget
- JavaScript-based styling that doesn't persist in the DOM

**Solution:**
Include all FastComments-related styles in a way that targets `.fastcomments-widget` or its child elements.

### Example: Complete Custom Theme

```css
/* Complete custom theme for FastComments + Profiles */
.fastcomments-widget {
    /* Colors */
    --fc-primary-color: #5469d4;
    --fc-secondary-color: #7c94f5;
    --fc-background-color: #ffffff;
    --fc-text-color: #1a1f36;
    --fc-border-color: #e3e8ee;

    /* Typography */
    --fc-font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    --fc-font-size: 15px;
    --fc-line-height: 1.6;

    /* Spacing */
    --fc-border-radius: 8px;

    /* Shadows */
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

/* Dark mode */
.dark-mode .fastcomments-widget {
    --fc-primary-color: #7c94f5;
    --fc-background-color: #1a1f36;
    --fc-text-color: #e3e8ee;
    --fc-border-color: #2d3748;
}

/* Profile-specific */
.fastcomments-profile-header {
    border-radius: var(--fc-border-radius) var(--fc-border-radius) 0 0;
}

.fastcomments-profile-badges .badge {
    background: linear-gradient(135deg, var(--fc-primary-color), var(--fc-secondary-color));
    color: white;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-weight: 600;
}
```

### Resources

For more information on customizing FastComments appearance:
- [Widget Customization Guide](/guides/customizations-and-configuration)
- [Styling Documentation](https://docs.fastcomments.com/styling)
- [Dark Mode Implementation](/guides/customizations-and-configuration/dark-mode)

### Summary

User Profile styling is seamless when opened from your widget:
1. Your widget's CSS automatically injects into profile modals
2. No additional configuration needed
3. Profiles inherit your custom theme
4. Consistent user experience across comments and profiles
5. Use CSS variables for easy theming
6. Test profiles whenever you update widget styles
