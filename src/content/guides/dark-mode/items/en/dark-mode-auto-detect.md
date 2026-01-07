By default, FastComments will auto-detect if your site has a dark background based on the "distance from black" in the color circle.

Our products try to do their best at this, however there are almost infinite colors in the color wheel, and there may be scenarios where the application
chooses to use dark mode when it is not appropriate, and vice-versa. This documentation covers how to have more fine-grained control over this.

#### Technical Details

We detect dark mode by traversing the elements in the page up from the comment widget, looking for a dark background when the widget initially loads.

To toggle dark mode after this step, you must call the widget to update its configuration. This is covered in the `Manual Configuration` section. 
