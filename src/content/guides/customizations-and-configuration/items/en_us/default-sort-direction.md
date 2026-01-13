[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

By default, FastComments sorts comments using the "Most Relevant" sort direction.

The Most Relevant sort takes into account when the comment was left and the number of votes.

Users can then change the sort direction to either Oldest First or Newest First in the comment widget UI.

However, you can change the default to any of the three. For example, if you want to show the oldest comments first:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

We set the value of **defaultSortDirection** to "OF" to set the direction to "OF".

For the newest-first sort direction, we would do the following:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

The valid values for **defaultSortDirection** are:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

You can also do this without code. On the widget customization page, see the "Default Sort Direction" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Note that the comments for each page and sort direction are pre-computed, so all sort directions have the same performance.