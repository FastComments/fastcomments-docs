Ahora que hemos descargado el archivo zip, extráelo a una carpeta. He descargado el `casper.zip` predeterminado y lo he extraído en `Downloads\casper` en Windows.

A continuación, asegúrate de tener instalada la versión LTS o una más reciente de NodeJS. Puedes obtenerla aquí: https://nodejs.org/en/download/

Una vez que NodeJS esté instalado, querrás instalar un editor de código.

Recomendamos (y usamos) Webstorm, que puedes obtener aquí con una prueba de 30 días (no se necesita tarjeta de crédito): https://www.jetbrains.com/webstorm/

La siguiente mejor opción gratuita probablemente sería Visual Studio Code: https://code.visualstudio.com/download

Una vez que tengas tu editor configurado y la carpeta del tema abierta en el editor, abre la terminal en el IDE y ejecuta:

[inline-code-attrs-start title = 'Instalar el tema'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

La salida exitosa se verá así (puedes ignorar las advertencias):

<div class="screenshot white-bg">
    <div class="title">Salida exitosa de npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Salida exitosa de npm install" />
</div>

Esto configurará las dependencias del tema para los comandos posteriores que ejecutaremos. Además, la exportación depende de que las dependencias del tema estén instaladas; de lo contrario, la reimportación no funcionará correctamente.