#!/bin/sh

TMP=/c/Temp/inpost-data
POSTER=/c/Users/Public/Pictures/pieczatka-wide.jpg
FILE=$(readlink -f $1)

mkdir -p $TMP
cd $TMP

magick convert \
	-verbose \
	-trim \
	-density 150 \
	$FILE \
	-quality 100 \
	-sharpen 0x1.0 \
	page.png

function create_page() {
	file=$1
	idx=$2
	pad=$((554*$(($idx-1))))
	echo =========$idx=$pad

	magick convert -quality 100 -crop 1240x554+0+$((30+$pad)) $file.out $file.$idx
	i=$(magick identify -format "%[standard-deviation]" $file.$idx)
	[ "2.6974e-06" != "$i" ] && \
		magick convert -append $file.$idx $POSTER $file.a.$idx
}

for f in page-*.png; do
	magick convert $f -background white -flatten -alpha off $f.out
	for i in `seq 1 3`; do
		create_page $f $i
	done
done

magick convert page-*.png.a.* -page A4+0+190 etykieta-do-druku.pdf
