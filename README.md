# fastcomments-docs

Documentation and Tutorials that lives at https://docs.fastcomments.com

## Contributing

Before starting work, please open an issue describing the improvement you want to make so that your work is not
duplicated by accident.

When you have something to show - create a pull request!

*Read the Conventions section below!*

## Setup

This project uses a small framework written in NodeJS. It requires Node 10 or newer.

    npm install

### Setup - Windows
    
    npm build-windows

### Setup - Linux/Unix

    npm build-posix

### Run Development Watch Job

    npm run watch


### Adding Content

1. In `src/content/guides`, add a folder.
2. Follow the pattern of the other folders for the file structure (you'll need an `items` directory and a `meta.json` for example).

#### Items Directory

The `items` directory is where the content for your guide lives.

#### meta.json

This is a file that describes your guide. Changing it will cause the watch job to rebuild the entire guide.

    {
      "name": "Moderation",
      "icon": "moderator.png",
      "itemsOrdered": [
        {
          "name": "The Moderate Comments Page",
          "file": "moderate-comments-page.md",
          "subCat": "Introduction"
        },
        ...

"subCat" is not referencing any identifier. Simply adding a subCat will create that sub category.

## Conventions

### Screenshots

In order to keep this documentation easy to maintain, we do *absolutely never hard code screenshots*.

Instead, we generate them, for example in your `some-guide-name/items/some-item.md` file:

    [app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

This will create a screenshot of `.max-comment-size` on the page `https://fastcomments.com/auth/my-account/customize-widget/new`, and title it `Limit Comment Length`.

The related code is in `src/app-screenshot-generator`.

### Code Snippets

WHen writing code snippets, it's helpful that you also highlight the section of the code snippet that is important.

    [code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

In this example, we create a code snippet of the VanillaJS widget and highlight lines 6.

The related code is in `src/code-example-generator.js`.

When updating the `Customizations & Configuration` documentation, please ensure that you list the parameter you are documenting at
the start of your documentation, to be consistent:

    [related-parameter-start name = 'headerHTML'; type = 'string'; related-parameter-end]

In our code snippets, we prefer to use TypeScript when possible as it is very verbose and easy to read, and if not then JavaScript.

Other languages are allowed when trying to illustrate examples in those languages.

### Grammar

We suggest using an editor with built in helpers for grammar.
