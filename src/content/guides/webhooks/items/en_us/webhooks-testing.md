In the Webhooks admin there are `Send Test Payload` buttons for each event type (Create, Update, Delete). The Create and Update events send a dummy WebhookComment object, while testing Delete will send a dummy request body with just an ID.

The test will make two calls to verify the response code for "happy" (correct API Key) and "sad" (invalid API key) scenarios.

When the test sends an invalid API key you should return a status code of 401 for the test to pass completely. If you don't correctly check the value of the token, you'll see an error.

This is to ensure that you properly authenticate the request.