העלאת תמונה ושינוי גודל

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | קביעת גודל: "Default" (1000x1000px) או "CrossPlatform" (יוצר גדלים למכשירים פופולריים) |
| urlId | string | query | No | מזהה העמוד שממנו מתבצע ההעלאה, להגדרה |

## תגובה

מחזיר: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## דוגמה

[inline-code-attrs-start title = 'דוגמה של upload_image'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# הגדרת ה-host היא אופציונלית והקבועה היא https://fastcomments.com
# ראו configuration.py לרשימת כל פרמטרי התצורה הנתמכים.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# הכנסו לקונטקסט עם מופע של לקוח ה-API
with client.ApiClient(configuration) as api_client:
    # צרו מופע של מחלקת ה-API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | קביעת גודל: \"Default\" (1000x1000px) או \"CrossPlatform\" (יוצר גדלים למכשירים פופולריים) (אופציונלי)
    url_id = 'url_id_example' # str | מזהה העמוד שממנו מתבצע ההעלאה, להגדרה (אופציונלי)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]