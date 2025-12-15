[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

By default, FastComments will sort comments by the "Most Relevant" sort direction.

Most Relevant sorting takes the time the comment was left, and the number of votes into account for sorting.

The user can then change the sort direction to either Oldest or Newest First in the comment widget UI.

However, we can change the default to be any of the three. For example if you wanted to show the oldest comments first:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

We set the value of **defaultSortDirection** to "OF" to set the direction to "OF".

For the newest-first sort direction, we would do the following:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

The valid values for **defaultSortDirection** are:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

This can also be done without code. In the widget customization page, see the "Default Sort Direction" section.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

Note that, the comments on each page for each sort direction are pre-computed, so all sort directions have the same performance.
