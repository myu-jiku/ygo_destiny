"""
YGO Destiny – A Yu-Gi-Oh! sealed draft simulator written in rust.
Copyright (C) 2022  myujiku

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License version 3 as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
"""

import project, xml
from gtk_xml import (
    GtkChildObject,
    GtkMargins,
    GtkObject,
    GtkProperty,
    GtkTrue,
    GtkFalse,
)
from xml import XmlString, XmlTag

class_name = "YGOUpdatePage"
parent_class = "AdwBin"

progress_bar = GtkChildObject(
    "GtkProgressBar",
    id="progress_bar",
    content=(
        GtkProperty("show-text", GtkTrue),
        GtkProperty("fraction", XmlString("0.25")),
    ),
)


container = GtkProperty(
    "child",
    GtkObject(
        "AdwClamp",
        (
            GtkChildObject(
                "GtkBox",
                (
                    *GtkMargins(12, 12, 12, 12),
                    GtkProperty("orientation", XmlString("vertical")),
                    GtkProperty("vexpand", GtkTrue),
                    GtkProperty("hexpand", GtkTrue),
                    GtkProperty("valign", XmlString("center")),
                    progress_bar,
                    GtkChildObject(
                        "GtkLabel",
                        (
                            GtkProperty("label", XmlString("this is a progress bar")),
                            # GtkProperty("halign", XmlString("start")),
                        ),
                    ),
                ),
            ),
        ),
    ),
)


gtk_template = XmlTag(
    "template",
    options={
        "class": class_name,
        "parent": parent_class,
    },
    content=container,
)

xml_content = XmlTag("interface", gtk_template)

xml.save_to_file(
    f"{project.directory}/{__name__.split('.')[-1]}.ui", xml_content, project.header
)
