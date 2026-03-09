#### Open Developer Keys in Canvas

Log in to Canvas as an administrator. Go to **Admin** (in the left sidebar) > select your account > **Developer Keys**.

#### Create an LTI Developer Key

Click **+ Developer Key** and select **LTI Key**.

In the configuration form:

1. In the **Redirect URIs** field (left side), paste the **Launch URL** from the FastComments setup page.
2. On the right, set **Method** to **Enter URL**.
3. Paste the **Configuration URL** you copied from FastComments into the **JSON URL** field.
4. Canvas will load the LTI configuration automatically.
5. Give the key a name (e.g. "FastComments").
6. Click **Save**.

#### Enable the Developer Key

After saving, the new key will appear in the Developer Keys table with its **State** set to **OFF**. Click the toggle to set it to **ON**. Canvas may prompt you to confirm — click **Allow** to enable the key.

#### Copy the Client ID

The Developer Keys table shows a numeric **Client ID** in the Details column (e.g. `17000000000042`). Copy this number - you will need it in the next step.
