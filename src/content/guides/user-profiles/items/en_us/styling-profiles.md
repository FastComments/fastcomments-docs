When User Profiles are opened in the context of your site (via the comment widget), any custom CSS styles you've applied to your FastComments widget are automatically injected into the profile modal.

### How It Works

When a user clicks on a profile link from your comment widget, a profile modal opens with the class `.fast-comments-profile`. Your widget's custom CSS is automatically injected into the profile view. If you've already styled your comment widget, those styles will apply to profiles.

### CSS Classes

FastComments profiles use a class-based CSS architecture. It does not use CSS custom properties.

The main profile page uses `.user-profile` as the root container. The header section is `.profile-header` with `.profile-header-background` for the background image. Profile content sits in `.profile-content`.

The avatar uses `.profile-avatar` and `.profile-avatar-wrapper`. The user's name is `.profile-name` and bio text is `.profile-bio`. Statistics are in `.profile-stats` with individual stats using `.stat`.

Social links are in `.profile-social-links` with individual links as `.social-link`. Badges use `.profile-badges` and `.badge`. Badge progress bars use `.progress-outer` and `.progress-bar`.

Tabs use `.profile-tabs` for the container, `.tab` for individual tabs, and `.tab.active` for the selected tab. Tab content uses `.tab-body` and `.tab-body.active`. Notification counts on tabs use `.tab .count`.

Notifications use `.notification` and DM conversations use `.conversation`. Online status is `.activity-indicator` with `.activity-indicator.online` for the active state. Unread counters use `.unread-count`.

The profile modal container is `.fast-comments-profile` with `.fast-comments-profile-close` for the close button.

### Dark Mode

Dark mode uses the `.dark` class modifier on `.user-profile`.

```css
.user-profile.dark {
    background-color: #181a1b;
    color: #fff;
}
```

### Examples

**Header:**
```css
.user-profile .profile-header-background {
    background: linear-gradient(to right, #667eea, #764ba2);
}
```

**Badges:**
```css
.user-profile .badge {
    background: #007bff;
    color: white;
    border-radius: 24px;
}
```

**Tabs:**
```css
.user-profile .tab.active {
    color: #007bff;
    border-bottom: 3px solid #007bff;
}
```

**Modal:**
```css
.fast-comments-profile {
    border-radius: 12px 0 0 12px;
}
```