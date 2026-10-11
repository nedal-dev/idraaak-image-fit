"""Check that the built portable library and publishing metadata agree."""
import os
import re
import xml.etree.ElementTree as ET
from pathlib import Path
from zipfile import ZipFile

ns = {"m": "http://maven.apache.org/POM/4.0.0"}
pom = ET.parse("pom.xml").getroot()

def value(path):
    return pom.findtext(path, namespaces=ns)

assert value("m:groupId") == "io.github.nedal-dev"
assert value("m:artifactId") == "idraaak-image-fit"
assert value("m:version") == "0.1.0"
assert value("m:url") == "https://idraaak.com/"
assert value("m:scm/m:url") == "https://github.com/nedal-dev/idraaak-image-fit"
revision = value("m:scm/m:tag")
assert revision and re.fullmatch(r"[0-9a-f]{40}", revision), revision
expected = os.environ.get("GITHUB_SHA")
if expected:
    assert revision == expected, (revision, expected)
assert value("m:licenses/m:license/m:name") == "MIT License"

jar = Path("target/idraaak-image-fit-0.1.0.jar")
with ZipFile(jar) as archive:
    names = set(archive.namelist())
    assert "idraaak/image_fit.cljc" in names
    assert "META-INF/LICENSE" in names
    assert not any("test_runner" in name or "image_fit_test" in name for name in names)
    embedded = ET.fromstring(
        archive.read("META-INF/maven/io.github.nedal-dev/idraaak-image-fit/pom.xml")
    )
    assert embedded.findtext("m:url", namespaces=ns) == "https://idraaak.com/"
    assert embedded.findtext("m:scm/m:tag", namespaces=ns) == revision
print("Release verified: portable source, MIT license, clean homepage, exact Git revision.")
