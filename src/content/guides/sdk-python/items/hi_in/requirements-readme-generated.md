- Python >= 3.8

बेस इंस्टॉल प्योर-stdlib है और SSO यूटिलिटीज़ प्रदान करता है। जनरेटेड
API क्लाइंट (`DefaultApi`/`PublicApi`/`ModerationApi`) को `client` एक्स्ट्रा की आवश्यकता होती है,
जो `urllib3 >= 1.25.3`, `python-dateutil >= 2.8.2`,
`pydantic >= 2.0.0`, और `typing-extensions >= 4.0.0` इंस्टॉल करता है:

```bash
pip install "fastcomments[client] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```