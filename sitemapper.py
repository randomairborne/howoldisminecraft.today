#!/usr/bin/python3
import json
import requests
javascriptobjectnotation = requests.get("https://launchermeta.mojang.com/mc/game/version_manifest.json")
mcdict = json.loads(javascriptobjectnotation.text)
mcdict = mcdict['versions']
with open("sitemap.xml", "w+") as xml:
    xml.write("""<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">""")
for entry in mcdict:
  with open("sitemap.xml", "a") as xml:
    xml.write(f"""
    <url>
        <loc>https://howoldisminecraft.today/{entry['id']}</loc>
        <lastmod>{entry['releaseTime'].split('T')[0]}</lastmod>
    </url>""")
with open("sitemap.xml", "a") as xml:
    xml.write("\n</urlset>")
