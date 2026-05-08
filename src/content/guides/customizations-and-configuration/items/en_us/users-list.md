[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

By default, FastComments does not show a list of users on the page.

You can render a list of people who are currently viewing the page, alongside the comment widget. The list updates live as users join and leave, and shows their name, avatar, and an online indicator.

There are three layout options:

- `1` - Top: a horizontal row of overlapping avatars rendered above the comments.
- `2` - Left: a sidebar with names and online dots rendered to the left of the widget.
- `3` - Right: the same sidebar rendered to the right of the widget.

Set the **usersListLocation** flag to enable the feature:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

By default the list shows only users currently online. To also include people who have commented on the page in the past (but are not currently viewing it), set **usersListIncludeOffline** to true:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

Past commenters are rendered without the green online dot so it is clear who is present right now.

Users with private profiles are shown with a generic avatar and a "Private Profile" label so the count remains accurate without revealing identities.

This can also be configured without code. In the widget customization page, see the "Users List Location" option. When the location is set to anything other than Off, an "Include past commenters" checkbox appears below it.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

Past 500 live users, the list is up to 30 seconds out of date.
