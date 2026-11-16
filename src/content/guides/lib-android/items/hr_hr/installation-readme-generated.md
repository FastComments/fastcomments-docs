Dodajte FastComments SDK u datoteku `build.gradle.kts` vaše aplikacije:

```kotlin
dependencies {
    implementation("com.fastcomments:android-sdk:2.1.0")
}
```

Provjerite imate li Repsy repozitorij u datoteci `settings.gradle.kts` vašeg projekta:

```kotlin
dependencyResolutionManagement {
    repositories {
        maven {
            url = uri("https://repo.repsy.io/mvn/winrid/fastcomments")
        }
        // other repositories...
    }
}
```