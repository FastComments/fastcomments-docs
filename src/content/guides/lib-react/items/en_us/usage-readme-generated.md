### The Main Widget Component

The FastCommentsCommentWidget component contains the live FastComments comment widget.

Replace "demo" below with your "tenantId" - available [here](https://fastcomments.com/auth/my-account/api) in the FastComments admin area.

The widget supports a lot of options - see FastCommentsCommentWidgetConfig in src/index.tsx.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentWidget tenantId="demo" />
  }
}
```

### Updating The Current Page (For SPAs)
To update the page/article the comment thread is tied to you must update the configuration parameters "urlId" and "url".
See the example and explanation [here](https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx).

### Account Region (ATTENTION: EU Customers)

If you're in the EU, you'll want to tell the client widgets what region you are in. See [examples/example-eu](/examples/example-eu/src/App.tsx);
Otherwise, you do not have to define `region`.

### The Comment Count Widget

The FastCommentsCommentCountWidget component contains the live FastComments comment count widget.

Replace "demo" below with your "tenantId" - available [here](https://fastcomments.com/auth/my-account/api) in the FastComments admin area.

See FastCommentsCommentCountConfig in src/index.tsx for the supported configuration options.

```tsx
import React, { Component } from 'react'

import {FastCommentsCommentCountWidget} from 'fastcomments-react'

class Example extends Component {
  render() {
    return <FastCommentsCommentCountWidget tenantId="demo" urlId="https://example.com/some-page-or-id" />
  }
}
```

### Native

For a completely native implementation of FastComments, see [fastcomments-react-native-sdk](https://github.com/FastComments/fastcomments-react-native-sdk).

For a React Native wrapper of this library, using a webview, see [fastcomments-react-native](https://github.com/FastComments/fastcomments-react-native).