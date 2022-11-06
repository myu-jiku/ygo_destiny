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

from pathlib import Path

app_id = "com.myujiku.ygo_destiny"
app_path = "/com/myujiku/ygo_destiny/"

directory: str = Path(__file__).absolute().parent.parent / "resources/templates"
header: str = """<?xml version="1.0" encoding="UTF-8"?>"""

if __name__ == "__main__":
    from ui import *
