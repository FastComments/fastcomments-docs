## How to Customize Comment Widget Styles

You can customize the comment widget styling in two ways:

### Option 1: Via customCSS Parameter

Pass your custom CSS as a string to the `customCSS` parameter when initializing the widget:

```javascript
window.fcConfigs = [{
    target: '#fastcomments-widget',
    tenantId: 'your-tenant-id',
    customCSS: `
        .fast-comments .comment {
            background-color: #f0f0f0 !important;
            border-radius: 8px !important;
        }
    `
}];
```

### Option 2: Via Admin Dashboard

1. Go to the [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget) in your admin dashboard
2. Scroll to the "Custom CSS" section under "Advanced"
3. Enter your custom CSS
4. Click "Save"

Your custom CSS will be applied to all comment widgets on your site.

## Tips

- Use `!important` to override default styles if needed
- Target specific selectors to avoid affecting other parts of your site
- Test your CSS in different browsers for compatibility
- The widget uses standard CSS - no special preprocessors required
