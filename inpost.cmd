REM @echo off
rmdir /s /q C:\Users\Praca\AppData\Local\Temp\seven_scissors
del C:\Users\Praca\AppData\Local\Temp\page-*-*.png
del C:\Users\Praca\AppData\Local\Temp\page-*-*.png.ele
mkdir C:\Users\Praca\AppData\Local\Temp\seven_scissors

"C:\Program Files\gs\gs9.55.0\bin\gswin64c.exe" -dNOPAUSE -dBATCH -sDEVICE=png16m -g2000x2828 -dPDFFitPage -dUseCropBox -sOutputFile="C:\Users\Praca\AppData\Local\Temp\seven_scissors\page-%%d.png" %1

seven_scissors C:\Users\Praca\AppData\Local\Temp\seven_scissors

for %%f in (C:\Users\Praca\AppData\Local\Temp\page-*-*.png) do (
	magick convert -append %%f C:\Users\Public\Pictures\pieczatka-wide.jpg %%f.ele
)

magick convert C:\Users\Praca\AppData\Local\Temp\page-*-*.png.ele  -page A4+0+410 C:\Users\Praca\AppData\Local\Temp\etykieta-do-druku.pdf

C:\Users\Praca\AppData\Local\Temp\etykieta-do-druku.pdf

