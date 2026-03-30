use crate::domain::item::{dto::*, item_code::ItemCode};

pub fn item_info(code: ItemCode) -> Option<&'static ItemRow> {
    match code {
        ItemCode::WA101 => Some(&ITEM_WA101),
        ItemCode::WA102 => Some(&ITEM_WA102),
        ItemCode::WA103 => Some(&ITEM_WA103),
        ItemCode::WA104 => Some(&ITEM_WA104),
        ItemCode::WA105 => Some(&ITEM_WA105),
        ItemCode::WA106 => Some(&ITEM_WA106),
        ItemCode::WA107 => Some(&ITEM_WA107),
        ItemCode::WA108 => Some(&ITEM_WA108),
        ItemCode::WA109 => Some(&ITEM_WA109),
        ItemCode::WA110 => Some(&ITEM_WA110),
        ItemCode::WA111 => Some(&ITEM_WA111),
        ItemCode::WA112 => Some(&ITEM_WA112),
        ItemCode::WA113 => Some(&ITEM_WA113),
        ItemCode::WA114 => Some(&ITEM_WA114),
        ItemCode::WA115 => Some(&ITEM_WA115),
        ItemCode::WC101 => Some(&ITEM_WC101),
        ItemCode::WC102 => Some(&ITEM_WC102),
        ItemCode::WC103 => Some(&ITEM_WC103),
        ItemCode::WC104 => Some(&ITEM_WC104),
        ItemCode::WC105 => Some(&ITEM_WC105),
        ItemCode::WC106 => Some(&ITEM_WC106),
        ItemCode::WC107 => Some(&ITEM_WC107),
        ItemCode::WC108 => Some(&ITEM_WC108),
        ItemCode::WC109 => Some(&ITEM_WC109),
        ItemCode::WC110 => Some(&ITEM_WC110),
        ItemCode::WC111 => Some(&ITEM_WC111),
        ItemCode::WC112 => Some(&ITEM_WC112),
        ItemCode::WC113 => Some(&ITEM_WC113),
        ItemCode::WC114 => Some(&ITEM_WC114),
        ItemCode::WC115 => Some(&ITEM_WC115),
        ItemCode::WH101 => Some(&ITEM_WH101),
        ItemCode::WH102 => Some(&ITEM_WH102),
        ItemCode::WH103 => Some(&ITEM_WH103),
        ItemCode::WH104 => Some(&ITEM_WH104),
        ItemCode::WH105 => Some(&ITEM_WH105),
        ItemCode::WH106 => Some(&ITEM_WH106),
        ItemCode::WH107 => Some(&ITEM_WH107),
        ItemCode::WH108 => Some(&ITEM_WH108),
        ItemCode::WH109 => Some(&ITEM_WH109),
        ItemCode::WH110 => Some(&ITEM_WH110),
        ItemCode::WH111 => Some(&ITEM_WH111),
        ItemCode::WH112 => Some(&ITEM_WH112),
        ItemCode::WH113 => Some(&ITEM_WH113),
        ItemCode::WH114 => Some(&ITEM_WH114),
        ItemCode::WH115 => Some(&ITEM_WH115),
        ItemCode::WH116 => Some(&ITEM_WH116),
        ItemCode::WM101 => Some(&ITEM_WM101),
        ItemCode::WM102 => Some(&ITEM_WM102),
        ItemCode::WM103 => Some(&ITEM_WM103),
        ItemCode::WM104 => Some(&ITEM_WM104),
        ItemCode::WM105 => Some(&ITEM_WM105),
        ItemCode::WM106 => Some(&ITEM_WM106),
        ItemCode::WM107 => Some(&ITEM_WM107),
        ItemCode::WM108 => Some(&ITEM_WM108),
        ItemCode::WM109 => Some(&ITEM_WM109),
        ItemCode::WM110 => Some(&ITEM_WM110),
        ItemCode::WM111 => Some(&ITEM_WM111),
        ItemCode::WM112 => Some(&ITEM_WM112),
        ItemCode::WM113 => Some(&ITEM_WM113),
        ItemCode::WM114 => Some(&ITEM_WM114),
        ItemCode::WM115 => Some(&ITEM_WM115),
        ItemCode::WM116 => Some(&ITEM_WM116),
        ItemCode::WP101 => Some(&ITEM_WP101),
        ItemCode::WP102 => Some(&ITEM_WP102),
        ItemCode::WP103 => Some(&ITEM_WP103),
        ItemCode::WP104 => Some(&ITEM_WP104),
        ItemCode::WP105 => Some(&ITEM_WP105),
        ItemCode::WP106 => Some(&ITEM_WP106),
        ItemCode::WP107 => Some(&ITEM_WP107),
        ItemCode::WP108 => Some(&ITEM_WP108),
        ItemCode::WP109 => Some(&ITEM_WP109),
        ItemCode::WP110 => Some(&ITEM_WP110),
        ItemCode::WP111 => Some(&ITEM_WP111),
        ItemCode::WP112 => Some(&ITEM_WP112),
        ItemCode::WP113 => Some(&ITEM_WP113),
        ItemCode::WP114 => Some(&ITEM_WP114),
        ItemCode::WP115 => Some(&ITEM_WP115),
        ItemCode::WP116 => Some(&ITEM_WP116),
        ItemCode::WS101 => Some(&ITEM_WS101),
        ItemCode::WS102 => Some(&ITEM_WS102),
        ItemCode::WS103 => Some(&ITEM_WS103),
        ItemCode::WS104 => Some(&ITEM_WS104),
        ItemCode::WS105 => Some(&ITEM_WS105),
        ItemCode::WS106 => Some(&ITEM_WS106),
        ItemCode::WS107 => Some(&ITEM_WS107),
        ItemCode::WS108 => Some(&ITEM_WS108),
        ItemCode::WS109 => Some(&ITEM_WS109),
        ItemCode::WS110 => Some(&ITEM_WS110),
        ItemCode::WS111 => Some(&ITEM_WS111),
        ItemCode::WS112 => Some(&ITEM_WS112),
        ItemCode::WS113 => Some(&ITEM_WS113),
        ItemCode::WS114 => Some(&ITEM_WS114),
        ItemCode::WS115 => Some(&ITEM_WS115),
        ItemCode::WS116 => Some(&ITEM_WS116),
        ItemCode::WS117 => Some(&ITEM_WS117),
        ItemCode::WS201 => Some(&ITEM_WS201),
        ItemCode::WS202 => Some(&ITEM_WS202),
        ItemCode::WS203 => Some(&ITEM_WS203),
        ItemCode::WS204 => Some(&ITEM_WS204),
        ItemCode::WS205 => Some(&ITEM_WS205),
        ItemCode::WS206 => Some(&ITEM_WS206),
        ItemCode::WS207 => Some(&ITEM_WS207),
        ItemCode::WS208 => Some(&ITEM_WS208),
        ItemCode::WS209 => Some(&ITEM_WS209),
        ItemCode::WS210 => Some(&ITEM_WS210),
        ItemCode::WS211 => Some(&ITEM_WS211),
        ItemCode::WS212 => Some(&ITEM_WS212),
        ItemCode::WS213 => Some(&ITEM_WS213),
        ItemCode::WS214 => Some(&ITEM_WS214),
        ItemCode::WS215 => Some(&ITEM_WS215),
        ItemCode::WS216 => Some(&ITEM_WS216),
        ItemCode::WS217 => Some(&ITEM_WS217),
        ItemCode::WS218 => Some(&ITEM_WS218),
        ItemCode::WT101 => Some(&ITEM_WT101),
        ItemCode::WT102 => Some(&ITEM_WT102),
        ItemCode::WT103 => Some(&ITEM_WT103),
        ItemCode::WT104 => Some(&ITEM_WT104),
        ItemCode::WT105 => Some(&ITEM_WT105),
        ItemCode::WT106 => Some(&ITEM_WT106),
        ItemCode::WT107 => Some(&ITEM_WT107),
        ItemCode::WT108 => Some(&ITEM_WT108),
        ItemCode::WT109 => Some(&ITEM_WT109),
        ItemCode::WT110 => Some(&ITEM_WT110),
        ItemCode::WT111 => Some(&ITEM_WT111),
        ItemCode::WT112 => Some(&ITEM_WT112),
        ItemCode::WT113 => Some(&ITEM_WT113),
        ItemCode::WT114 => Some(&ITEM_WT114),
        ItemCode::WT115 => Some(&ITEM_WT115),
        ItemCode::WT116 => Some(&ITEM_WT116),
        ItemCode::WA116 => Some(&ITEM_WA116),
        ItemCode::WA117 => Some(&ITEM_WA117),
        ItemCode::WA118 => Some(&ITEM_WA118),
        ItemCode::WA119 => Some(&ITEM_WA119),
        ItemCode::WA120 => Some(&ITEM_WA120),
        ItemCode::WA121 => Some(&ITEM_WA121),
        ItemCode::WA122 => Some(&ITEM_WA122),
        ItemCode::WA123 => Some(&ITEM_WA123),
        ItemCode::WA124 => Some(&ITEM_WA124),
        ItemCode::WA125 => Some(&ITEM_WA125),
        ItemCode::WA126 => Some(&ITEM_WA126),
        ItemCode::WA127 => Some(&ITEM_WA127),
        ItemCode::WA128 => Some(&ITEM_WA128),
        ItemCode::WA129 => Some(&ITEM_WA129),
        ItemCode::WA130 => Some(&ITEM_WA130),
        ItemCode::WA131 => Some(&ITEM_WA131),
        ItemCode::WC116 => Some(&ITEM_WC116),
        ItemCode::WC117 => Some(&ITEM_WC117),
        ItemCode::WC118 => Some(&ITEM_WC118),
        ItemCode::WC119 => Some(&ITEM_WC119),
        ItemCode::WC120 => Some(&ITEM_WC120),
        ItemCode::WC121 => Some(&ITEM_WC121),
        ItemCode::WC122 => Some(&ITEM_WC122),
        ItemCode::WC123 => Some(&ITEM_WC123),
        ItemCode::WC124 => Some(&ITEM_WC124),
        ItemCode::WC125 => Some(&ITEM_WC125),
        ItemCode::WC126 => Some(&ITEM_WC126),
        ItemCode::WC127 => Some(&ITEM_WC127),
        ItemCode::WC128 => Some(&ITEM_WC128),
        ItemCode::WC129 => Some(&ITEM_WC129),
        ItemCode::WC130 => Some(&ITEM_WC130),
        ItemCode::WH117 => Some(&ITEM_WH117),
        ItemCode::WH118 => Some(&ITEM_WH118),
        ItemCode::WH119 => Some(&ITEM_WH119),
        ItemCode::WH120 => Some(&ITEM_WH120),
        ItemCode::WH121 => Some(&ITEM_WH121),
        ItemCode::WH122 => Some(&ITEM_WH122),
        ItemCode::WH123 => Some(&ITEM_WH123),
        ItemCode::WH124 => Some(&ITEM_WH124),
        ItemCode::WH125 => Some(&ITEM_WH125),
        ItemCode::WH126 => Some(&ITEM_WH126),
        ItemCode::WH127 => Some(&ITEM_WH127),
        ItemCode::WH128 => Some(&ITEM_WH128),
        ItemCode::WH129 => Some(&ITEM_WH129),
        ItemCode::WH130 => Some(&ITEM_WH130),
        ItemCode::WH131 => Some(&ITEM_WH131),
        ItemCode::WH132 => Some(&ITEM_WH132),
        ItemCode::WM117 => Some(&ITEM_WM117),
        ItemCode::WM118 => Some(&ITEM_WM118),
        ItemCode::WM119 => Some(&ITEM_WM119),
        ItemCode::WM120 => Some(&ITEM_WM120),
        ItemCode::WM121 => Some(&ITEM_WM121),
        ItemCode::WM122 => Some(&ITEM_WM122),
        ItemCode::WM123 => Some(&ITEM_WM123),
        ItemCode::WM124 => Some(&ITEM_WM124),
        ItemCode::WM125 => Some(&ITEM_WM125),
        ItemCode::WM126 => Some(&ITEM_WM126),
        ItemCode::WM127 => Some(&ITEM_WM127),
        ItemCode::WM128 => Some(&ITEM_WM128),
        ItemCode::WM129 => Some(&ITEM_WM129),
        ItemCode::WM130 => Some(&ITEM_WM130),
        ItemCode::WM131 => Some(&ITEM_WM131),
        ItemCode::WM132 => Some(&ITEM_WM132),
        ItemCode::WP117 => Some(&ITEM_WP117),
        ItemCode::WP118 => Some(&ITEM_WP118),
        ItemCode::WP119 => Some(&ITEM_WP119),
        ItemCode::WP120 => Some(&ITEM_WP120),
        ItemCode::WP121 => Some(&ITEM_WP121),
        ItemCode::WP122 => Some(&ITEM_WP122),
        ItemCode::WP123 => Some(&ITEM_WP123),
        ItemCode::WP124 => Some(&ITEM_WP124),
        ItemCode::WP125 => Some(&ITEM_WP125),
        ItemCode::WP126 => Some(&ITEM_WP126),
        ItemCode::WP127 => Some(&ITEM_WP127),
        ItemCode::WP128 => Some(&ITEM_WP128),
        ItemCode::WP129 => Some(&ITEM_WP129),
        ItemCode::WP130 => Some(&ITEM_WP130),
        ItemCode::WP131 => Some(&ITEM_WP131),
        ItemCode::WP132 => Some(&ITEM_WP132),
        ItemCode::WP133 => Some(&ITEM_WP133),
        ItemCode::WP134 => Some(&ITEM_WP134),
        ItemCode::WP135 => Some(&ITEM_WP135),
        ItemCode::WP136 => Some(&ITEM_WP136),
        ItemCode::WS118 => Some(&ITEM_WS118),
        ItemCode::WS119 => Some(&ITEM_WS119),
        ItemCode::WS120 => Some(&ITEM_WS120),
        ItemCode::WS121 => Some(&ITEM_WS121),
        ItemCode::WS122 => Some(&ITEM_WS122),
        ItemCode::WS123 => Some(&ITEM_WS123),
        ItemCode::WS124 => Some(&ITEM_WS124),
        ItemCode::WS125 => Some(&ITEM_WS125),
        ItemCode::WS126 => Some(&ITEM_WS126),
        ItemCode::WS127 => Some(&ITEM_WS127),
        ItemCode::WS128 => Some(&ITEM_WS128),
        ItemCode::WS129 => Some(&ITEM_WS129),
        ItemCode::WS130 => Some(&ITEM_WS130),
        ItemCode::WS131 => Some(&ITEM_WS131),
        ItemCode::WS132 => Some(&ITEM_WS132),
        ItemCode::WS219 => Some(&ITEM_WS219),
        ItemCode::WS220 => Some(&ITEM_WS220),
        ItemCode::WS221 => Some(&ITEM_WS221),
        ItemCode::WS222 => Some(&ITEM_WS222),
        ItemCode::WS223 => Some(&ITEM_WS223),
        ItemCode::WS224 => Some(&ITEM_WS224),
        ItemCode::WS225 => Some(&ITEM_WS225),
        ItemCode::WS226 => Some(&ITEM_WS226),
        ItemCode::WS227 => Some(&ITEM_WS227),
        ItemCode::WS228 => Some(&ITEM_WS228),
        ItemCode::WS229 => Some(&ITEM_WS229),
        ItemCode::WS230 => Some(&ITEM_WS230),
        ItemCode::WS231 => Some(&ITEM_WS231),
        ItemCode::WS232 => Some(&ITEM_WS232),
        ItemCode::WS233 => Some(&ITEM_WS233),
        ItemCode::WS234 => Some(&ITEM_WS234),
        ItemCode::WS251 => Some(&ITEM_WS251),
        ItemCode::WT117 => Some(&ITEM_WT117),
        ItemCode::WT118 => Some(&ITEM_WT118),
        ItemCode::WT119 => Some(&ITEM_WT119),
        ItemCode::WT120 => Some(&ITEM_WT120),
        ItemCode::WT121 => Some(&ITEM_WT121),
        ItemCode::WT122 => Some(&ITEM_WT122),
        ItemCode::WT123 => Some(&ITEM_WT123),
        ItemCode::WT124 => Some(&ITEM_WT124),
        ItemCode::WT125 => Some(&ITEM_WT125),
        ItemCode::WT126 => Some(&ITEM_WT126),
        ItemCode::WT127 => Some(&ITEM_WT127),
        ItemCode::WT128 => Some(&ITEM_WT128),
        ItemCode::WT129 => Some(&ITEM_WT129),
        ItemCode::WT130 => Some(&ITEM_WT130),
        ItemCode::WT131 => Some(&ITEM_WT131),
        ItemCode::OA101 => Some(&ITEM_OA101),
        ItemCode::OA102 => Some(&ITEM_OA102),
        ItemCode::OA103 => Some(&ITEM_OA103),
        ItemCode::OA104 => Some(&ITEM_OA104),
        ItemCode::OA105 => Some(&ITEM_OA105),
        ItemCode::OA106 => Some(&ITEM_OA106),
        ItemCode::OA107 => Some(&ITEM_OA107),
        ItemCode::OA108 => Some(&ITEM_OA108),
        ItemCode::OA109 => Some(&ITEM_OA109),
        ItemCode::OA110 => Some(&ITEM_OA110),
        ItemCode::OA111 => Some(&ITEM_OA111),
        ItemCode::OA112 => Some(&ITEM_OA112),
        ItemCode::OA113 => Some(&ITEM_OA113),
        ItemCode::OA114 => Some(&ITEM_OA114),
        ItemCode::OA115 => Some(&ITEM_OA115),
        ItemCode::OA116 => Some(&ITEM_OA116),
        ItemCode::OA117 => Some(&ITEM_OA117),
        ItemCode::OA118 => Some(&ITEM_OA118),
        ItemCode::OA119 => Some(&ITEM_OA119),
        ItemCode::OA120 => Some(&ITEM_OA120),
        ItemCode::OA121 => Some(&ITEM_OA121),
        ItemCode::OA122 => Some(&ITEM_OA122),
        ItemCode::OA123 => Some(&ITEM_OA123),
        ItemCode::OA124 => Some(&ITEM_OA124),
        ItemCode::OA125 => Some(&ITEM_OA125),
        ItemCode::OA126 => Some(&ITEM_OA126),
        ItemCode::OA129 => Some(&ITEM_OA129),
        ItemCode::OA130 => Some(&ITEM_OA130),
        ItemCode::OA131 => Some(&ITEM_OA131),
        ItemCode::OA132 => Some(&ITEM_OA132),
        ItemCode::OA133 => Some(&ITEM_OA133),
        ItemCode::OA134 => Some(&ITEM_OA134),
        ItemCode::OA135 => Some(&ITEM_OA135),
        ItemCode::OA136 => Some(&ITEM_OA136),
        ItemCode::OA137 => Some(&ITEM_OA137),
        ItemCode::OA138 => Some(&ITEM_OA138),
        ItemCode::OA139 => Some(&ITEM_OA139),
        ItemCode::OA140 => Some(&ITEM_OA140),
        ItemCode::OA141 => Some(&ITEM_OA141),
        ItemCode::OA142 => Some(&ITEM_OA142),
        ItemCode::OA201 => Some(&ITEM_OA201),
        ItemCode::OA202 => Some(&ITEM_OA202),
        ItemCode::WN101 => Some(&ITEM_WN101),
        ItemCode::WN102 => Some(&ITEM_WN102),
        ItemCode::WN103 => Some(&ITEM_WN103),
        ItemCode::WN104 => Some(&ITEM_WN104),
        ItemCode::WN105 => Some(&ITEM_WN105),
        ItemCode::WN106 => Some(&ITEM_WN106),
        ItemCode::WN107 => Some(&ITEM_WN107),
        ItemCode::WN108 => Some(&ITEM_WN108),
        ItemCode::WN109 => Some(&ITEM_WN109),
        ItemCode::WN110 => Some(&ITEM_WN110),
        ItemCode::WN111 => Some(&ITEM_WN111),
        ItemCode::WN112 => Some(&ITEM_WN112),
        ItemCode::WN113 => Some(&ITEM_WN113),
        ItemCode::WN114 => Some(&ITEM_WN114),
        ItemCode::WN115 => Some(&ITEM_WN115),
        ItemCode::WN116 => Some(&ITEM_WN116),
        ItemCode::WN117 => Some(&ITEM_WN117),
        ItemCode::WN118 => Some(&ITEM_WN118),
        ItemCode::WN119 => Some(&ITEM_WN119),
        ItemCode::WN120 => Some(&ITEM_WN120),
        ItemCode::WN121 => Some(&ITEM_WN121),
        ItemCode::WN122 => Some(&ITEM_WN122),
        ItemCode::WN123 => Some(&ITEM_WN123),
        ItemCode::WN124 => Some(&ITEM_WN124),
        ItemCode::WN125 => Some(&ITEM_WN125),
        ItemCode::WN126 => Some(&ITEM_WN126),
        ItemCode::WN127 => Some(&ITEM_WN127),
        ItemCode::WN128 => Some(&ITEM_WN128),
        ItemCode::WN129 => Some(&ITEM_WN129),
        ItemCode::WN130 => Some(&ITEM_WN130),
        ItemCode::WN152 => Some(&ITEM_WN152),
        ItemCode::WN153 => Some(&ITEM_WN153),
        ItemCode::WN154 => Some(&ITEM_WN154),
        ItemCode::WN155 => Some(&ITEM_WN155),
        ItemCode::WN156 => Some(&ITEM_WN156),
        ItemCode::WD101 => Some(&ITEM_WD101),
        ItemCode::WD102 => Some(&ITEM_WD102),
        ItemCode::WD103 => Some(&ITEM_WD103),
        ItemCode::WD104 => Some(&ITEM_WD104),
        ItemCode::WD105 => Some(&ITEM_WD105),
        ItemCode::WD106 => Some(&ITEM_WD106),
        ItemCode::WD107 => Some(&ITEM_WD107),
        ItemCode::WD108 => Some(&ITEM_WD108),
        ItemCode::WD109 => Some(&ITEM_WD109),
        ItemCode::WD110 => Some(&ITEM_WD110),
        ItemCode::WD111 => Some(&ITEM_WD111),
        ItemCode::WD112 => Some(&ITEM_WD112),
        ItemCode::WD113 => Some(&ITEM_WD113),
        ItemCode::WD114 => Some(&ITEM_WD114),
        ItemCode::WD115 => Some(&ITEM_WD115),
        ItemCode::WD116 => Some(&ITEM_WD116),
        ItemCode::WD117 => Some(&ITEM_WD117),
        ItemCode::WD118 => Some(&ITEM_WD118),
        ItemCode::WD119 => Some(&ITEM_WD119),
        ItemCode::WD120 => Some(&ITEM_WD120),
        ItemCode::WD121 => Some(&ITEM_WD121),
        ItemCode::WD122 => Some(&ITEM_WD122),
        ItemCode::WD123 => Some(&ITEM_WD123),
        ItemCode::WD124 => Some(&ITEM_WD124),
        ItemCode::WD125 => Some(&ITEM_WD125),
        ItemCode::WD126 => Some(&ITEM_WD126),
        ItemCode::WD127 => Some(&ITEM_WD127),
        ItemCode::WD128 => Some(&ITEM_WD128),
        ItemCode::WD129 => Some(&ITEM_WD129),
        ItemCode::WD130 => Some(&ITEM_WD130),
        ItemCode::WD152 => Some(&ITEM_WD152),
        ItemCode::WD153 => Some(&ITEM_WD153),
        ItemCode::WD154 => Some(&ITEM_WD154),
        ItemCode::WD155 => Some(&ITEM_WD155),
        ItemCode::WD156 => Some(&ITEM_WD156),
        ItemCode::WV101 => Some(&ITEM_WV101),
        ItemCode::WV102 => Some(&ITEM_WV102),
        ItemCode::WV103 => Some(&ITEM_WV103),
        ItemCode::WV104 => Some(&ITEM_WV104),
        ItemCode::WV105 => Some(&ITEM_WV105),
        ItemCode::WV106 => Some(&ITEM_WV106),
        ItemCode::WV107 => Some(&ITEM_WV107),
        ItemCode::WV108 => Some(&ITEM_WV108),
        ItemCode::WV109 => Some(&ITEM_WV109),
        ItemCode::WV110 => Some(&ITEM_WV110),
        ItemCode::WV111 => Some(&ITEM_WV111),
        ItemCode::WV112 => Some(&ITEM_WV112),
        ItemCode::WV113 => Some(&ITEM_WV113),
        ItemCode::WV114 => Some(&ITEM_WV114),
        ItemCode::WV115 => Some(&ITEM_WV115),
        ItemCode::WV116 => Some(&ITEM_WV116),
        ItemCode::WV117 => Some(&ITEM_WV117),
        ItemCode::WV118 => Some(&ITEM_WV118),
        ItemCode::WV119 => Some(&ITEM_WV119),
        ItemCode::WV120 => Some(&ITEM_WV120),
        ItemCode::WV121 => Some(&ITEM_WV121),
        ItemCode::WV122 => Some(&ITEM_WV122),
        ItemCode::WV123 => Some(&ITEM_WV123),
        ItemCode::WV124 => Some(&ITEM_WV124),
        ItemCode::WV125 => Some(&ITEM_WV125),
        ItemCode::WV126 => Some(&ITEM_WV126),
        ItemCode::WV127 => Some(&ITEM_WV127),
        ItemCode::WV128 => Some(&ITEM_WV128),
        ItemCode::WV129 => Some(&ITEM_WV129),
        ItemCode::WV130 => Some(&ITEM_WV130),
        ItemCode::OA203 => Some(&ITEM_OA203),
        ItemCode::OA204 => Some(&ITEM_OA204),
        ItemCode::OA205 => Some(&ITEM_OA205),
        ItemCode::OA206 => Some(&ITEM_OA206),
        ItemCode::OA207 => Some(&ITEM_OA207),
        ItemCode::OA208 => Some(&ITEM_OA208),
        ItemCode::OA209 => Some(&ITEM_OA209),
        ItemCode::OA210 => Some(&ITEM_OA210),
        ItemCode::OA211 => Some(&ITEM_OA211),
        ItemCode::OA212 => Some(&ITEM_OA212),
        ItemCode::OA213 => Some(&ITEM_OA213),
        ItemCode::OA214 => Some(&ITEM_OA214),
        ItemCode::OA215 => Some(&ITEM_OA215),
        ItemCode::OA216 => Some(&ITEM_OA216),
        ItemCode::OA217 => Some(&ITEM_OA217),
        ItemCode::OA218 => Some(&ITEM_OA218),
        ItemCode::OA219 => Some(&ITEM_OA219),
        ItemCode::OA220 => Some(&ITEM_OA220),
        ItemCode::OA221 => Some(&ITEM_OA221),
        ItemCode::OA222 => Some(&ITEM_OA222),
        ItemCode::OA223 => Some(&ITEM_OA223),
        ItemCode::OA224 => Some(&ITEM_OA224),
        ItemCode::OA225 => Some(&ITEM_OA225),
        ItemCode::OA226 => Some(&ITEM_OA226),
        ItemCode::OA227 => Some(&ITEM_OA227),
        ItemCode::OA228 => Some(&ITEM_OA228),
        ItemCode::OA229 => Some(&ITEM_OA229),
        ItemCode::OA231 => Some(&ITEM_OA231),
        ItemCode::OA232 => Some(&ITEM_OA232),
        ItemCode::OA233 => Some(&ITEM_OA233),
        ItemCode::OA234 => Some(&ITEM_OA234),
        ItemCode::OA251 => Some(&ITEM_OA251),
        ItemCode::OA252 => Some(&ITEM_OA252),
        ItemCode::OM101 => Some(&ITEM_OM101),
        ItemCode::OM102 => Some(&ITEM_OM102),
        ItemCode::OM103 => Some(&ITEM_OM103),
        ItemCode::OM104 => Some(&ITEM_OM104),
        ItemCode::OM105 => Some(&ITEM_OM105),
        ItemCode::OM106 => Some(&ITEM_OM106),
        ItemCode::OM107 => Some(&ITEM_OM107),
        ItemCode::OM108 => Some(&ITEM_OM108),
        ItemCode::OM109 => Some(&ITEM_OM109),
        ItemCode::OM110 => Some(&ITEM_OM110),
        ItemCode::OM111 => Some(&ITEM_OM111),
        ItemCode::OM112 => Some(&ITEM_OM112),
        ItemCode::OM113 => Some(&ITEM_OM113),
        ItemCode::OM114 => Some(&ITEM_OM114),
        ItemCode::OM115 => Some(&ITEM_OM115),
        ItemCode::OM116 => Some(&ITEM_OM116),
        ItemCode::OM117 => Some(&ITEM_OM117),
        ItemCode::OM118 => Some(&ITEM_OM118),
        ItemCode::OM119 => Some(&ITEM_OM119),
        ItemCode::OM120 => Some(&ITEM_OM120),
        ItemCode::OM121 => Some(&ITEM_OM121),
        ItemCode::OM122 => Some(&ITEM_OM122),
        ItemCode::OM123 => Some(&ITEM_OM123),
        ItemCode::OM124 => Some(&ITEM_OM124),
        ItemCode::OM125 => Some(&ITEM_OM125),
        ItemCode::OM126 => Some(&ITEM_OM126),
        ItemCode::OM127 => Some(&ITEM_OM127),
        ItemCode::OM128 => Some(&ITEM_OM128),
        ItemCode::OM129 => Some(&ITEM_OM129),
        ItemCode::OM130 => Some(&ITEM_OM130),
        ItemCode::OM131 => Some(&ITEM_OM131),
        ItemCode::OR101 => Some(&ITEM_OR101),
        ItemCode::OR102 => Some(&ITEM_OR102),
        ItemCode::OR103 => Some(&ITEM_OR103),
        ItemCode::OR104 => Some(&ITEM_OR104),
        ItemCode::OR105 => Some(&ITEM_OR105),
        ItemCode::OR106 => Some(&ITEM_OR106),
        ItemCode::OR107 => Some(&ITEM_OR107),
        ItemCode::OR108 => Some(&ITEM_OR108),
        ItemCode::OR109 => Some(&ITEM_OR109),
        ItemCode::OR110 => Some(&ITEM_OR110),
        ItemCode::OR111 => Some(&ITEM_OR111),
        ItemCode::OR112 => Some(&ITEM_OR112),
        ItemCode::OR113 => Some(&ITEM_OR113),
        ItemCode::OR114 => Some(&ITEM_OR114),
        ItemCode::OR115 => Some(&ITEM_OR115),
        ItemCode::OR116 => Some(&ITEM_OR116),
        ItemCode::OR117 => Some(&ITEM_OR117),
        ItemCode::OR118 => Some(&ITEM_OR118),
        ItemCode::OR119 => Some(&ITEM_OR119),
        ItemCode::OR120 => Some(&ITEM_OR120),
        ItemCode::OA127 => Some(&ITEM_OA127),
        ItemCode::OA144 => Some(&ITEM_OA144),
        ItemCode::OR121 => Some(&ITEM_OR121),
        ItemCode::OR122 => Some(&ITEM_OR122),
        ItemCode::OR123 => Some(&ITEM_OR123),
        ItemCode::OR124 => Some(&ITEM_OR124),
        ItemCode::OR125 => Some(&ITEM_OR125),
        ItemCode::OR129 => Some(&ITEM_OR129),
        ItemCode::OS101 => Some(&ITEM_OS101),
        ItemCode::OS102 => Some(&ITEM_OS102),
        ItemCode::OS103 => Some(&ITEM_OS103),
        ItemCode::OS104 => Some(&ITEM_OS104),
        ItemCode::OS105 => Some(&ITEM_OS105),
        ItemCode::OS106 => Some(&ITEM_OS106),
        ItemCode::OS107 => Some(&ITEM_OS107),
        ItemCode::OS108 => Some(&ITEM_OS108),
        ItemCode::OS109 => Some(&ITEM_OS109),
        ItemCode::OS110 => Some(&ITEM_OS110),
        ItemCode::OS111 => Some(&ITEM_OS111),
        ItemCode::OS112 => Some(&ITEM_OS112),
        ItemCode::OS113 => Some(&ITEM_OS113),
        ItemCode::OS114 => Some(&ITEM_OS114),
        ItemCode::OS115 => Some(&ITEM_OS115),
        ItemCode::OS116 => Some(&ITEM_OS116),
        ItemCode::OS117 => Some(&ITEM_OS117),
        ItemCode::OS121 => Some(&ITEM_OS121),
        ItemCode::OS122 => Some(&ITEM_OS122),
        ItemCode::OS123 => Some(&ITEM_OS123),
        ItemCode::OS124 => Some(&ITEM_OS124),
        ItemCode::OS125 => Some(&ITEM_OS125),
        ItemCode::OS126 => Some(&ITEM_OS126),
        ItemCode::OS127 => Some(&ITEM_OS127),
        ItemCode::OS128 => Some(&ITEM_OS128),
        ItemCode::OS129 => Some(&ITEM_OS129),
        ItemCode::OS130 => Some(&ITEM_OS130),
        ItemCode::OS131 => Some(&ITEM_OS131),
        ItemCode::OS132 => Some(&ITEM_OS132),
        ItemCode::OS133 => Some(&ITEM_OS133),
        ItemCode::OS134 => Some(&ITEM_OS134),
        ItemCode::FO101 => Some(&ITEM_FO101),
        ItemCode::FO102 => Some(&ITEM_FO102),
        ItemCode::FO103 => Some(&ITEM_FO103),
        ItemCode::FO104 => Some(&ITEM_FO104),
        ItemCode::FO105 => Some(&ITEM_FO105),
        ItemCode::FO106 => Some(&ITEM_FO106),
        ItemCode::FO107 => Some(&ITEM_FO107),
        ItemCode::FO108 => Some(&ITEM_FO108),
        ItemCode::FO109 => Some(&ITEM_FO109),
        ItemCode::FO110 => Some(&ITEM_FO110),
        ItemCode::FO111 => Some(&ITEM_FO111),
        ItemCode::FO112 => Some(&ITEM_FO112),
        ItemCode::FO113 => Some(&ITEM_FO113),
        ItemCode::FO114 => Some(&ITEM_FO114),
        ItemCode::FO121 => Some(&ITEM_FO121),
        ItemCode::FO122 => Some(&ITEM_FO122),
        ItemCode::FO123 => Some(&ITEM_FO123),
        ItemCode::FO124 => Some(&ITEM_FO124),
        ItemCode::FO125 => Some(&ITEM_FO125),
        ItemCode::FO126 => Some(&ITEM_FO126),
        ItemCode::FO127 => Some(&ITEM_FO127),
        ItemCode::FO128 => Some(&ITEM_FO128),
        ItemCode::FO129 => Some(&ITEM_FO129),
        ItemCode::FO130 => Some(&ITEM_FO130),
        ItemCode::FO131 => Some(&ITEM_FO131),
        ItemCode::FO132 => Some(&ITEM_FO132),
        ItemCode::FO133 => Some(&ITEM_FO133),
        ItemCode::FO134 => Some(&ITEM_FO134),
        ItemCode::FO135 => Some(&ITEM_FO135),
        ItemCode::FO136 => Some(&ITEM_FO136),
        ItemCode::FO137 => Some(&ITEM_FO137),
        ItemCode::DA101 => Some(&ITEM_DA101),
        ItemCode::DA102 => Some(&ITEM_DA102),
        ItemCode::DA103 => Some(&ITEM_DA103),
        ItemCode::DA104 => Some(&ITEM_DA104),
        ItemCode::DA105 => Some(&ITEM_DA105),
        ItemCode::DA106 => Some(&ITEM_DA106),
        ItemCode::DA107 => Some(&ITEM_DA107),
        ItemCode::DA108 => Some(&ITEM_DA108),
        ItemCode::DA109 => Some(&ITEM_DA109),
        ItemCode::DA110 => Some(&ITEM_DA110),
        ItemCode::DA111 => Some(&ITEM_DA111),
        ItemCode::DA112 => Some(&ITEM_DA112),
        ItemCode::DA113 => Some(&ITEM_DA113),
        ItemCode::DA114 => Some(&ITEM_DA114),
        ItemCode::DA115 => Some(&ITEM_DA115),
        ItemCode::DA116 => Some(&ITEM_DA116),
        ItemCode::DA117 => Some(&ITEM_DA117),
        ItemCode::DA118 => Some(&ITEM_DA118),
        ItemCode::DA119 => Some(&ITEM_DA119),
        ItemCode::DA120 => Some(&ITEM_DA120),
        ItemCode::DA121 => Some(&ITEM_DA121),
        ItemCode::DA122 => Some(&ITEM_DA122),
        ItemCode::DA123 => Some(&ITEM_DA123),
        ItemCode::DA124 => Some(&ITEM_DA124),
        ItemCode::DA125 => Some(&ITEM_DA125),
        ItemCode::DA126 => Some(&ITEM_DA126),
        ItemCode::DA127 => Some(&ITEM_DA127),
        ItemCode::DA128 => Some(&ITEM_DA128),
        ItemCode::DA129 => Some(&ITEM_DA129),
        ItemCode::DA301 => Some(&ITEM_DA301),
        ItemCode::DA302 => Some(&ITEM_DA302),
        ItemCode::DA303 => Some(&ITEM_DA303),
        ItemCode::DA304 => Some(&ITEM_DA304),
        ItemCode::DA305 => Some(&ITEM_DA305),
        ItemCode::DA306 => Some(&ITEM_DA306),
        ItemCode::DA307 => Some(&ITEM_DA307),
        ItemCode::DA308 => Some(&ITEM_DA308),
        ItemCode::DA201 => Some(&ITEM_DA201),
        ItemCode::DA202 => Some(&ITEM_DA202),
        ItemCode::DA203 => Some(&ITEM_DA203),
        ItemCode::DA204 => Some(&ITEM_DA204),
        ItemCode::DA205 => Some(&ITEM_DA205),
        ItemCode::DA206 => Some(&ITEM_DA206),
        ItemCode::DA207 => Some(&ITEM_DA207),
        ItemCode::DA208 => Some(&ITEM_DA208),
        ItemCode::DA209 => Some(&ITEM_DA209),
        ItemCode::DA210 => Some(&ITEM_DA210),
        ItemCode::DA211 => Some(&ITEM_DA211),
        ItemCode::DA212 => Some(&ITEM_DA212),
        ItemCode::DA213 => Some(&ITEM_DA213),
        ItemCode::DA214 => Some(&ITEM_DA214),
        ItemCode::DA215 => Some(&ITEM_DA215),
        ItemCode::DA216 => Some(&ITEM_DA216),
        ItemCode::DA217 => Some(&ITEM_DA217),
        ItemCode::DA218 => Some(&ITEM_DA218),
        ItemCode::DA219 => Some(&ITEM_DA219),
        ItemCode::DA220 => Some(&ITEM_DA220),
        ItemCode::DA221 => Some(&ITEM_DA221),
        ItemCode::DA222 => Some(&ITEM_DA222),
        ItemCode::DA223 => Some(&ITEM_DA223),
        ItemCode::DA224 => Some(&ITEM_DA224),
        ItemCode::DA225 => Some(&ITEM_DA225),
        ItemCode::DA226 => Some(&ITEM_DA226),
        ItemCode::DA227 => Some(&ITEM_DA227),
        ItemCode::DA228 => Some(&ITEM_DA228),
        ItemCode::DA229 => Some(&ITEM_DA229),
        ItemCode::DA401 => Some(&ITEM_DA401),
        ItemCode::DA402 => Some(&ITEM_DA402),
        ItemCode::DA403 => Some(&ITEM_DA403),
        ItemCode::DA404 => Some(&ITEM_DA404),
        ItemCode::DA405 => Some(&ITEM_DA405),
        ItemCode::DA406 => Some(&ITEM_DA406),
        ItemCode::DA407 => Some(&ITEM_DA407),
        ItemCode::DA408 => Some(&ITEM_DA408),
        ItemCode::DA131 => Some(&ITEM_DA131),
        ItemCode::DA132 => Some(&ITEM_DA132),
        ItemCode::DA133 => Some(&ITEM_DA133),
        ItemCode::DA134 => Some(&ITEM_DA134),
        ItemCode::DA135 => Some(&ITEM_DA135),
        ItemCode::DA136 => Some(&ITEM_DA136),
        ItemCode::DA137 => Some(&ITEM_DA137),
        ItemCode::DA138 => Some(&ITEM_DA138),
        ItemCode::DA139 => Some(&ITEM_DA139),
        ItemCode::DA140 => Some(&ITEM_DA140),
        ItemCode::DA141 => Some(&ITEM_DA141),
        ItemCode::DA142 => Some(&ITEM_DA142),
        ItemCode::DA143 => Some(&ITEM_DA143),
        ItemCode::DA144 => Some(&ITEM_DA144),
        ItemCode::DA145 => Some(&ITEM_DA145),
        ItemCode::DA146 => Some(&ITEM_DA146),
        ItemCode::DA231 => Some(&ITEM_DA231),
        ItemCode::DA232 => Some(&ITEM_DA232),
        ItemCode::DA233 => Some(&ITEM_DA233),
        ItemCode::DA234 => Some(&ITEM_DA234),
        ItemCode::DA235 => Some(&ITEM_DA235),
        ItemCode::DA236 => Some(&ITEM_DA236),
        ItemCode::DA237 => Some(&ITEM_DA237),
        ItemCode::DA238 => Some(&ITEM_DA238),
        ItemCode::DA239 => Some(&ITEM_DA239),
        ItemCode::DA240 => Some(&ITEM_DA240),
        ItemCode::DA241 => Some(&ITEM_DA241),
        ItemCode::DA242 => Some(&ITEM_DA242),
        ItemCode::DA243 => Some(&ITEM_DA243),
        ItemCode::DA244 => Some(&ITEM_DA244),
        ItemCode::DA245 => Some(&ITEM_DA245),
        ItemCode::DA246 => Some(&ITEM_DA246),
        ItemCode::DA147 => Some(&ITEM_DA147),
        ItemCode::DA247 => Some(&ITEM_DA247),
        ItemCode::DA148 => Some(&ITEM_DA148),
        ItemCode::DA248 => Some(&ITEM_DA248),
        ItemCode::DA149 => Some(&ITEM_DA149),
        ItemCode::DA249 => Some(&ITEM_DA249),
        ItemCode::DA150 => Some(&ITEM_DA150),
        ItemCode::DA250 => Some(&ITEM_DA250),
        ItemCode::DA151 => Some(&ITEM_DA151),
        ItemCode::DA251 => Some(&ITEM_DA251),
        ItemCode::DA152 => Some(&ITEM_DA152),
        ItemCode::DA252 => Some(&ITEM_DA252),
        ItemCode::DA153 => Some(&ITEM_DA153),
        ItemCode::DA253 => Some(&ITEM_DA253),
        ItemCode::DA254 => Some(&ITEM_DA254),
        ItemCode::DA255 => Some(&ITEM_DA255),
        ItemCode::DA162 => Some(&ITEM_DA162),
        ItemCode::DA163 => Some(&ITEM_DA163),
        ItemCode::DA262 => Some(&ITEM_DA262),
        ItemCode::DA263 => Some(&ITEM_DA263),
        ItemCode::DA176 => Some(&ITEM_DA176),
        ItemCode::DA177 => Some(&ITEM_DA177),
        ItemCode::DA276 => Some(&ITEM_DA276),
        ItemCode::DA277 => Some(&ITEM_DA277),
        ItemCode::DB101 => Some(&ITEM_DB101),
        ItemCode::DB102 => Some(&ITEM_DB102),
        ItemCode::DB103 => Some(&ITEM_DB103),
        ItemCode::DB104 => Some(&ITEM_DB104),
        ItemCode::DB105 => Some(&ITEM_DB105),
        ItemCode::DB106 => Some(&ITEM_DB106),
        ItemCode::DB107 => Some(&ITEM_DB107),
        ItemCode::DB108 => Some(&ITEM_DB108),
        ItemCode::DB109 => Some(&ITEM_DB109),
        ItemCode::DB110 => Some(&ITEM_DB110),
        ItemCode::DB111 => Some(&ITEM_DB111),
        ItemCode::DB112 => Some(&ITEM_DB112),
        ItemCode::DB113 => Some(&ITEM_DB113),
        ItemCode::DB114 => Some(&ITEM_DB114),
        ItemCode::DB115 => Some(&ITEM_DB115),
        ItemCode::DB116 => Some(&ITEM_DB116),
        ItemCode::DB117 => Some(&ITEM_DB117),
        ItemCode::DB118 => Some(&ITEM_DB118),
        ItemCode::DB119 => Some(&ITEM_DB119),
        ItemCode::DB120 => Some(&ITEM_DB120),
        ItemCode::DB121 => Some(&ITEM_DB121),
        ItemCode::DB122 => Some(&ITEM_DB122),
        ItemCode::DB123 => Some(&ITEM_DB123),
        ItemCode::DB124 => Some(&ITEM_DB124),
        ItemCode::DB125 => Some(&ITEM_DB125),
        ItemCode::DB126 => Some(&ITEM_DB126),
        ItemCode::DB127 => Some(&ITEM_DB127),
        ItemCode::DB128 => Some(&ITEM_DB128),
        ItemCode::DB129 => Some(&ITEM_DB129),
        ItemCode::DB130 => Some(&ITEM_DB130),
        ItemCode::DB131 => Some(&ITEM_DB131),
        ItemCode::DB132 => Some(&ITEM_DB132),
        ItemCode::DB133 => Some(&ITEM_DB133),
        ItemCode::DB134 => Some(&ITEM_DB134),
        ItemCode::DB150 => Some(&ITEM_DB150),
        ItemCode::DG101 => Some(&ITEM_DG101),
        ItemCode::DG102 => Some(&ITEM_DG102),
        ItemCode::DG103 => Some(&ITEM_DG103),
        ItemCode::DG104 => Some(&ITEM_DG104),
        ItemCode::DG105 => Some(&ITEM_DG105),
        ItemCode::DG106 => Some(&ITEM_DG106),
        ItemCode::DG107 => Some(&ITEM_DG107),
        ItemCode::DG108 => Some(&ITEM_DG108),
        ItemCode::DG109 => Some(&ITEM_DG109),
        ItemCode::DG110 => Some(&ITEM_DG110),
        ItemCode::DG111 => Some(&ITEM_DG111),
        ItemCode::DG112 => Some(&ITEM_DG112),
        ItemCode::DG113 => Some(&ITEM_DG113),
        ItemCode::DG114 => Some(&ITEM_DG114),
        ItemCode::DG115 => Some(&ITEM_DG115),
        ItemCode::DG116 => Some(&ITEM_DG116),
        ItemCode::DG117 => Some(&ITEM_DG117),
        ItemCode::DG118 => Some(&ITEM_DG118),
        ItemCode::DG119 => Some(&ITEM_DG119),
        ItemCode::DG120 => Some(&ITEM_DG120),
        ItemCode::DG121 => Some(&ITEM_DG121),
        ItemCode::DG122 => Some(&ITEM_DG122),
        ItemCode::DG123 => Some(&ITEM_DG123),
        ItemCode::DG124 => Some(&ITEM_DG124),
        ItemCode::DG125 => Some(&ITEM_DG125),
        ItemCode::DG126 => Some(&ITEM_DG126),
        ItemCode::DG127 => Some(&ITEM_DG127),
        ItemCode::DG128 => Some(&ITEM_DG128),
        ItemCode::DG129 => Some(&ITEM_DG129),
        ItemCode::DG130 => Some(&ITEM_DG130),
        ItemCode::DS101 => Some(&ITEM_DS101),
        ItemCode::DS102 => Some(&ITEM_DS102),
        ItemCode::DS103 => Some(&ITEM_DS103),
        ItemCode::DS104 => Some(&ITEM_DS104),
        ItemCode::DS105 => Some(&ITEM_DS105),
        ItemCode::DS106 => Some(&ITEM_DS106),
        ItemCode::DS107 => Some(&ITEM_DS107),
        ItemCode::DS108 => Some(&ITEM_DS108),
        ItemCode::DS109 => Some(&ITEM_DS109),
        ItemCode::DS110 => Some(&ITEM_DS110),
        ItemCode::DS111 => Some(&ITEM_DS111),
        ItemCode::DS112 => Some(&ITEM_DS112),
        ItemCode::DS113 => Some(&ITEM_DS113),
        ItemCode::DS114 => Some(&ITEM_DS114),
        ItemCode::DS115 => Some(&ITEM_DS115),
        ItemCode::DS116 => Some(&ITEM_DS116),
        ItemCode::DS117 => Some(&ITEM_DS117),
        ItemCode::DS118 => Some(&ITEM_DS118),
        ItemCode::DS119 => Some(&ITEM_DS119),
        ItemCode::DS120 => Some(&ITEM_DS120),
        ItemCode::DS121 => Some(&ITEM_DS121),
        ItemCode::DS122 => Some(&ITEM_DS122),
        ItemCode::DS123 => Some(&ITEM_DS123),
        ItemCode::DS124 => Some(&ITEM_DS124),
        ItemCode::DS125 => Some(&ITEM_DS125),
        ItemCode::DS126 => Some(&ITEM_DS126),
        ItemCode::DS127 => Some(&ITEM_DS127),
        ItemCode::DS128 => Some(&ITEM_DS128),
        ItemCode::DS130 => Some(&ITEM_DS130),
        ItemCode::OR201 => Some(&ITEM_OR201),
        ItemCode::OR202 => Some(&ITEM_OR202),
        ItemCode::OR203 => Some(&ITEM_OR203),
        ItemCode::OR204 => Some(&ITEM_OR204),
        ItemCode::OR205 => Some(&ITEM_OR205),
        ItemCode::OR206 => Some(&ITEM_OR206),
        ItemCode::OR207 => Some(&ITEM_OR207),
        ItemCode::OR208 => Some(&ITEM_OR208),
        ItemCode::OR209 => Some(&ITEM_OR209),
        ItemCode::OR210 => Some(&ITEM_OR210),
        ItemCode::OR211 => Some(&ITEM_OR211),
        ItemCode::OR212 => Some(&ITEM_OR212),
        ItemCode::OR213 => Some(&ITEM_OR213),
        ItemCode::OR214 => Some(&ITEM_OR214),
        ItemCode::OR215 => Some(&ITEM_OR215),
        ItemCode::OR216 => Some(&ITEM_OR216),
        ItemCode::OR217 => Some(&ITEM_OR217),
        ItemCode::OR218 => Some(&ITEM_OR218),
        ItemCode::OR219 => Some(&ITEM_OR219),
        ItemCode::OR220 => Some(&ITEM_OR220),
        ItemCode::OR221 => Some(&ITEM_OR221),
        ItemCode::OR222 => Some(&ITEM_OR222),
        ItemCode::OR223 => Some(&ITEM_OR223),
        ItemCode::OR224 => Some(&ITEM_OR224),
        ItemCode::OR225 => Some(&ITEM_OR225),
        ItemCode::OR227 => Some(&ITEM_OR227),
        ItemCode::OR228 => Some(&ITEM_OR228),
        ItemCode::OR229 => Some(&ITEM_OR229),
        ItemCode::OR230 => Some(&ITEM_OR230),
        ItemCode::OR231 => Some(&ITEM_OR231),
        ItemCode::OR232 => Some(&ITEM_OR232),
        ItemCode::OR233 => Some(&ITEM_OR233),
        ItemCode::OR234 => Some(&ITEM_OR234),
        ItemCode::OR235 => Some(&ITEM_OR235),
        ItemCode::OR236 => Some(&ITEM_OR236),
        ItemCode::OR237 => Some(&ITEM_OR237),
        ItemCode::OR238 => Some(&ITEM_OR238),
        ItemCode::OR239 => Some(&ITEM_OR239),
        ItemCode::OR240 => Some(&ITEM_OR240),
        ItemCode::OR248 => Some(&ITEM_OR248),
        ItemCode::OR251 => Some(&ITEM_OR251),
        ItemCode::PM101 => Some(&ITEM_PM101),
        ItemCode::PM102 => Some(&ITEM_PM102),
        ItemCode::PM103 => Some(&ITEM_PM103),
        ItemCode::PM104 => Some(&ITEM_PM104),
        ItemCode::PM105 => Some(&ITEM_PM105),
        ItemCode::PL101 => Some(&ITEM_PL101),
        ItemCode::PL102 => Some(&ITEM_PL102),
        ItemCode::PL103 => Some(&ITEM_PL103),
        ItemCode::PL104 => Some(&ITEM_PL104),
        ItemCode::PL105 => Some(&ITEM_PL105),
        ItemCode::PS101 => Some(&ITEM_PS101),
        ItemCode::PS102 => Some(&ITEM_PS102),
        ItemCode::PS103 => Some(&ITEM_PS103),
        ItemCode::PS104 => Some(&ITEM_PS104),
        ItemCode::PS105 => Some(&ITEM_PS105),
        ItemCode::EC101 => Some(&ITEM_EC101),
        ItemCode::EC102 => Some(&ITEM_EC102),
        ItemCode::EC103 => Some(&ITEM_EC103),
        ItemCode::EC104 => Some(&ITEM_EC104),
        ItemCode::EC105 => Some(&ITEM_EC105),
        ItemCode::EC106 => Some(&ITEM_EC106),
        ItemCode::QT101 => Some(&ITEM_QT101),
        ItemCode::QT102 => Some(&ITEM_QT102),
        ItemCode::QT103 => Some(&ITEM_QT103),
        ItemCode::QT104 => Some(&ITEM_QT104),
        ItemCode::QT105 => Some(&ITEM_QT105),
        ItemCode::QT106 => Some(&ITEM_QT106),
        ItemCode::QT107 => Some(&ITEM_QT107),
        ItemCode::QT108 => Some(&ITEM_QT108),
        ItemCode::QT109 => Some(&ITEM_QT109),
        ItemCode::QT110 => Some(&ITEM_QT110),
        ItemCode::QT111 => Some(&ITEM_QT111),
        ItemCode::QT112 => Some(&ITEM_QT112),
        ItemCode::QT113 => Some(&ITEM_QT113),
        ItemCode::QT114 => Some(&ITEM_QT114),
        ItemCode::QT115 => Some(&ITEM_QT115),
        ItemCode::QT116 => Some(&ITEM_QT116),
        ItemCode::SP101 => Some(&ITEM_SP101),
        ItemCode::SP102 => Some(&ITEM_SP102),
        ItemCode::SP103 => Some(&ITEM_SP103),
        ItemCode::SP105 => Some(&ITEM_SP105),
        ItemCode::SP106 => Some(&ITEM_SP106),
        ItemCode::SP107 => Some(&ITEM_SP107),
        ItemCode::SP108 => Some(&ITEM_SP108),
        ItemCode::SP109 => Some(&ITEM_SP109),
        ItemCode::SP110 => Some(&ITEM_SP110),
        ItemCode::SP115 => Some(&ITEM_SP115),
        ItemCode::SP126 => Some(&ITEM_SP126),
        ItemCode::SP127 => Some(&ITEM_SP127),
        ItemCode::SP128 => Some(&ITEM_SP128),
        ItemCode::SP129 => Some(&ITEM_SP129),
        ItemCode::SP130 => Some(&ITEM_SP130),
        ItemCode::SP131 => Some(&ITEM_SP131),
        ItemCode::SP132 => Some(&ITEM_SP132),
        ItemCode::SP133 => Some(&ITEM_SP133),
        ItemCode::SP134 => Some(&ITEM_SP134),
        ItemCode::SP135 => Some(&ITEM_SP135),
        ItemCode::SP136 => Some(&ITEM_SP136),
        ItemCode::SP137 => Some(&ITEM_SP137),
        ItemCode::SP138 => Some(&ITEM_SP138),
        ItemCode::SP139 => Some(&ITEM_SP139),
        ItemCode::SP140 => Some(&ITEM_SP140),
        ItemCode::SP142 => Some(&ITEM_SP142),
        ItemCode::SP143 => Some(&ITEM_SP143),
        ItemCode::SP144 => Some(&ITEM_SP144),
        ItemCode::SP145 => Some(&ITEM_SP145),
        ItemCode::SP146 => Some(&ITEM_SP146),
        ItemCode::SP147 => Some(&ITEM_SP147),
        ItemCode::SP148 => Some(&ITEM_SP148),
        ItemCode::SP149 => Some(&ITEM_SP149),
        ItemCode::SP150 => Some(&ITEM_SP150),
        ItemCode::SP151 => Some(&ITEM_SP151),
        ItemCode::SP152 => Some(&ITEM_SP152),
        ItemCode::SP153 => Some(&ITEM_SP153),
        ItemCode::SP154 => Some(&ITEM_SP154),
        ItemCode::SP155 => Some(&ITEM_SP155),
        ItemCode::SP156 => Some(&ITEM_SP156),
        ItemCode::SP157 => Some(&ITEM_SP157),
        ItemCode::SP160 => Some(&ITEM_SP160),
        ItemCode::SP161 => Some(&ITEM_SP161),
        ItemCode::SP162 => Some(&ITEM_SP162),
        ItemCode::SP163 => Some(&ITEM_SP163),
        ItemCode::GP101 => Some(&ITEM_GP101),
        ItemCode::GP102 => Some(&ITEM_GP102),
        ItemCode::GP103 => Some(&ITEM_GP103),
        ItemCode::GP104 => Some(&ITEM_GP104),
        ItemCode::GP105 => Some(&ITEM_GP105),
        ItemCode::GP106 => Some(&ITEM_GP106),
        ItemCode::GP107 => Some(&ITEM_GP107),
        ItemCode::GP108 => Some(&ITEM_GP108),
        ItemCode::GP109 => Some(&ITEM_GP109),
        ItemCode::GP110 => Some(&ITEM_GP110),
        ItemCode::GP111 => Some(&ITEM_GP111),
        ItemCode::GP112 => Some(&ITEM_GP112),
        ItemCode::GP113 => Some(&ITEM_GP113),
        ItemCode::GP114 => Some(&ITEM_GP114),
        ItemCode::GP115 => Some(&ITEM_GP115),
        ItemCode::GP116 => Some(&ITEM_GP116),
        ItemCode::GP117 => Some(&ITEM_GP117),
        ItemCode::GP118 => Some(&ITEM_GP118),
        ItemCode::GP119 => Some(&ITEM_GP119),
        ItemCode::GP120 => Some(&ITEM_GP120),
        ItemCode::GP121 => Some(&ITEM_GP121),
        ItemCode::GP122 => Some(&ITEM_GP122),
        ItemCode::GP135 => Some(&ITEM_GP135),
        ItemCode::GP136 => Some(&ITEM_GP136),
        ItemCode::GP137 => Some(&ITEM_GP137),
        ItemCode::GP201 => Some(&ITEM_GP201),
        ItemCode::GP202 => Some(&ITEM_GP202),
        ItemCode::GP203 => Some(&ITEM_GP203),
        ItemCode::GP204 => Some(&ITEM_GP204),
        ItemCode::GP205 => Some(&ITEM_GP205),
        ItemCode::GP206 => Some(&ITEM_GP206),
        ItemCode::GP207 => Some(&ITEM_GP207),
        ItemCode::GP208 => Some(&ITEM_GP208),
        ItemCode::GP209 => Some(&ITEM_GP209),
        ItemCode::GP210 => Some(&ITEM_GP210),
        ItemCode::GP211 => Some(&ITEM_GP211),
        ItemCode::QW101 => Some(&ITEM_QW101),
        ItemCode::QW102 => Some(&ITEM_QW102),
        ItemCode::QW103 => Some(&ITEM_QW103),
        ItemCode::QW104 => Some(&ITEM_QW104),
        ItemCode::QW105 => Some(&ITEM_QW105),
        ItemCode::QW106 => Some(&ITEM_QW106),
        ItemCode::MA101 => Some(&ITEM_MA101),
        ItemCode::MA201 => Some(&ITEM_MA201),
        ItemCode::MA202 => Some(&ITEM_MA202),
        ItemCode::GF101 => Some(&ITEM_GF101),
        ItemCode::GF103 => Some(&ITEM_GF103),
        ItemCode::GF104 => Some(&ITEM_GF104),
        ItemCode::GF105 => Some(&ITEM_GF105),
        ItemCode::GF106 => Some(&ITEM_GF106),
        ItemCode::GF107 => Some(&ITEM_GF107),
        ItemCode::GF108 => Some(&ITEM_GF108),
        ItemCode::GF102 => Some(&ITEM_GF102),
        ItemCode::SD201 => Some(&ITEM_SD201),
        ItemCode::SD202 => Some(&ITEM_SD202),
        ItemCode::SD203 => Some(&ITEM_SD203),
        ItemCode::SD204 => Some(&ITEM_SD204),
        ItemCode::SD205 => Some(&ITEM_SD205),
        ItemCode::SD206 => Some(&ITEM_SD206),
        ItemCode::SD207 => Some(&ITEM_SD207),
        ItemCode::BS101 => Some(&ITEM_BS101),
        ItemCode::BS102 => Some(&ITEM_BS102),
        ItemCode::BS103 => Some(&ITEM_BS103),
        ItemCode::BC101 => Some(&ITEM_BC101),
        ItemCode::BC102 => Some(&ITEM_BC102),
        ItemCode::BC103 => Some(&ITEM_BC103),
        ItemCode::BC104 => Some(&ITEM_BC104),
        ItemCode::BC105 => Some(&ITEM_BC105),
        ItemCode::BC106 => Some(&ITEM_BC106),
        ItemCode::BC107 => Some(&ITEM_BC107),
        ItemCode::BC108 => Some(&ITEM_BC108),
        ItemCode::BC109 => Some(&ITEM_BC109),
        ItemCode::BC110 => Some(&ITEM_BC110),
        ItemCode::BC111 => Some(&ITEM_BC111),
        ItemCode::BC112 => Some(&ITEM_BC112),
        ItemCode::BC113 => Some(&ITEM_BC113),
        ItemCode::BC114 => Some(&ITEM_BC114),
        ItemCode::BC115 => Some(&ITEM_BC115),
        ItemCode::BC116 => Some(&ITEM_BC116),
        ItemCode::BC121 => Some(&ITEM_BC121),
        ItemCode::BC122 => Some(&ITEM_BC122),
        ItemCode::BC123 => Some(&ITEM_BC123),
        ItemCode::BC124 => Some(&ITEM_BC124),
        ItemCode::BC125 => Some(&ITEM_BC125),
        ItemCode::BC126 => Some(&ITEM_BC126),
        ItemCode::BC127 => Some(&ITEM_BC127),
        ItemCode::BC128 => Some(&ITEM_BC128),
        ItemCode::BC129 => Some(&ITEM_BC129),
        ItemCode::BC130 => Some(&ITEM_BC130),
        ItemCode::BC131 => Some(&ITEM_BC131),
        ItemCode::BC132 => Some(&ITEM_BC132),
        ItemCode::BI101 => Some(&ITEM_BI101),
        ItemCode::BI102 => Some(&ITEM_BI102),
        ItemCode::BI103 => Some(&ITEM_BI103),
        ItemCode::BI104 => Some(&ITEM_BI104),
        ItemCode::BI105 => Some(&ITEM_BI105),
        ItemCode::BI106 => Some(&ITEM_BI106),
        ItemCode::BI107 => Some(&ITEM_BI107),
        ItemCode::BI108 => Some(&ITEM_BI108),
        ItemCode::BI109 => Some(&ITEM_BI109),
        ItemCode::BI110 => Some(&ITEM_BI110),
        ItemCode::BI111 => Some(&ITEM_BI111),
        ItemCode::BI112 => Some(&ITEM_BI112),
        ItemCode::BI113 => Some(&ITEM_BI113),
        ItemCode::BI114 => Some(&ITEM_BI114),
        ItemCode::BI115 => Some(&ITEM_BI115),
        ItemCode::BI116 => Some(&ITEM_BI116),
        ItemCode::BI117 => Some(&ITEM_BI117),
        ItemCode::BI118 => Some(&ITEM_BI118),
        ItemCode::BI119 => Some(&ITEM_BI119),
        ItemCode::BI120 => Some(&ITEM_BI120),
        ItemCode::BI121 => Some(&ITEM_BI121),
        ItemCode::BI122 => Some(&ITEM_BI122),
        ItemCode::BI123 => Some(&ITEM_BI123),
        ItemCode::BI124 => Some(&ITEM_BI124),
        ItemCode::BI125 => Some(&ITEM_BI125),
        ItemCode::BI126 => Some(&ITEM_BI126),
        ItemCode::BI127 => Some(&ITEM_BI127),
        ItemCode::BI128 => Some(&ITEM_BI128),
        ItemCode::BI129 => Some(&ITEM_BI129),
        ItemCode::BI130 => Some(&ITEM_BI130),
        ItemCode::BI131 => Some(&ITEM_BI131),
        ItemCode::BI132 => Some(&ITEM_BI132),
        ItemCode::BI133 => Some(&ITEM_BI133),
        ItemCode::BI134 => Some(&ITEM_BI134),
        ItemCode::BI136 => Some(&ITEM_BI136),
        ItemCode::BI137 => Some(&ITEM_BI137),
        ItemCode::BI138 => Some(&ITEM_BI138),
        ItemCode::BI139 => Some(&ITEM_BI139),
        ItemCode::BI140 => Some(&ITEM_BI140),
        ItemCode::BI141 => Some(&ITEM_BI141),
        ItemCode::BI142 => Some(&ITEM_BI142),
        ItemCode::BI143 => Some(&ITEM_BI143),
        ItemCode::BI144 => Some(&ITEM_BI144),
        ItemCode::BI145 => Some(&ITEM_BI145),
        ItemCode::BI146 => Some(&ITEM_BI146),
        ItemCode::BI147 => Some(&ITEM_BI147),
        ItemCode::BI148 => Some(&ITEM_BI148),
        ItemCode::BI149 => Some(&ITEM_BI149),
        ItemCode::BI150 => Some(&ITEM_BI150),
        ItemCode::BI151 => Some(&ITEM_BI151),
        ItemCode::BI152 => Some(&ITEM_BI152),
        ItemCode::BI153 => Some(&ITEM_BI153),
        ItemCode::BI154 => Some(&ITEM_BI154),
        ItemCode::BI155 => Some(&ITEM_BI155),
        ItemCode::BI160 => Some(&ITEM_BI160),
        ItemCode::BI161 => Some(&ITEM_BI161),
        ItemCode::BI162 => Some(&ITEM_BI162),
        ItemCode::BI163 => Some(&ITEM_BI163),
        ItemCode::BI164 => Some(&ITEM_BI164),
        ItemCode::BI165 => Some(&ITEM_BI165),
        ItemCode::BI166 => Some(&ITEM_BI166),
        ItemCode::BI167 => Some(&ITEM_BI167),
        ItemCode::BI168 => Some(&ITEM_BI168),
        ItemCode::BI169 => Some(&ITEM_BI169),
        ItemCode::BI170 => Some(&ITEM_BI170),
        ItemCode::BI171 => Some(&ITEM_BI171),
        ItemCode::BI172 => Some(&ITEM_BI172),
        ItemCode::BI173 => Some(&ITEM_BI173),
        ItemCode::BI174 => Some(&ITEM_BI174),
        ItemCode::BI175 => Some(&ITEM_BI175),
        ItemCode::BI176 => Some(&ITEM_BI176),
        ItemCode::BI177 => Some(&ITEM_BI177),
        ItemCode::BI178 => Some(&ITEM_BI178),
        ItemCode::BI179 => Some(&ITEM_BI179),
        ItemCode::BI180 => Some(&ITEM_BI180),
        ItemCode::BI181 => Some(&ITEM_BI181),
        ItemCode::BI182 => Some(&ITEM_BI182),
        ItemCode::BI183 => Some(&ITEM_BI183),
        ItemCode::BI184 => Some(&ITEM_BI184),
        ItemCode::BI185 => Some(&ITEM_BI185),
        ItemCode::BI186 => Some(&ITEM_BI186),
        ItemCode::BI187 => Some(&ITEM_BI187),
        ItemCode::BI188 => Some(&ITEM_BI188),
        ItemCode::BI189 => Some(&ITEM_BI189),
        ItemCode::BI190 => Some(&ITEM_BI190),
        ItemCode::BI191 => Some(&ITEM_BI191),
        ItemCode::BI192 => Some(&ITEM_BI192),
        ItemCode::BI193 => Some(&ITEM_BI193),
        ItemCode::BI195 => Some(&ITEM_BI195),
        ItemCode::BI196 => Some(&ITEM_BI196),
        ItemCode::BI197 => Some(&ITEM_BI197),
        ItemCode::BI198 => Some(&ITEM_BI198),
        ItemCode::BI201 => Some(&ITEM_BI201),
        ItemCode::BI202 => Some(&ITEM_BI202),
        ItemCode::BI203 => Some(&ITEM_BI203),
        ItemCode::BI204 => Some(&ITEM_BI204),
        ItemCode::BI205 => Some(&ITEM_BI205),
        ItemCode::BI206 => Some(&ITEM_BI206),
        ItemCode::BI207 => Some(&ITEM_BI207),
        ItemCode::BI208 => Some(&ITEM_BI208),
        ItemCode::BI209 => Some(&ITEM_BI209),
        ItemCode::BI210 => Some(&ITEM_BI210),
        ItemCode::BI211 => Some(&ITEM_BI211),
        ItemCode::BI212 => Some(&ITEM_BI212),
        ItemCode::BI213 => Some(&ITEM_BI213),
        ItemCode::BI214 => Some(&ITEM_BI214),
        ItemCode::BI215 => Some(&ITEM_BI215),
        ItemCode::BI216 => Some(&ITEM_BI216),
        ItemCode::BI217 => Some(&ITEM_BI217),
        ItemCode::BI218 => Some(&ITEM_BI218),
        ItemCode::BI219 => Some(&ITEM_BI219),
        ItemCode::BI220 => Some(&ITEM_BI220),
        ItemCode::BI221 => Some(&ITEM_BI221),
        ItemCode::BI222 => Some(&ITEM_BI222),
        ItemCode::BI223 => Some(&ITEM_BI223),
        ItemCode::BI224 => Some(&ITEM_BI224),
        ItemCode::BI225 => Some(&ITEM_BI225),
        ItemCode::BI226 => Some(&ITEM_BI226),
        ItemCode::BI227 => Some(&ITEM_BI227),
        ItemCode::BI228 => Some(&ITEM_BI228),
        ItemCode::BI229 => Some(&ITEM_BI229),
        ItemCode::BI230 => Some(&ITEM_BI230),
        ItemCode::BI231 => Some(&ITEM_BI231),
        ItemCode::BI232 => Some(&ITEM_BI232),
        ItemCode::BI233 => Some(&ITEM_BI233),
        ItemCode::BI234 => Some(&ITEM_BI234),
        ItemCode::BI235 => Some(&ITEM_BI235),
        ItemCode::BI236 => Some(&ITEM_BI236),
        ItemCode::BI237 => Some(&ITEM_BI237),
        ItemCode::BI238 => Some(&ITEM_BI238),
        ItemCode::BI239 => Some(&ITEM_BI239),
        ItemCode::BI240 => Some(&ITEM_BI240),
        ItemCode::BI241 => Some(&ITEM_BI241),
        ItemCode::BI242 => Some(&ITEM_BI242),
        ItemCode::BI243 => Some(&ITEM_BI243),
        ItemCode::BI244 => Some(&ITEM_BI244),
        ItemCode::BI245 => Some(&ITEM_BI245),
        ItemCode::BI246 => Some(&ITEM_BI246),
        ItemCode::BI247 => Some(&ITEM_BI247),
        ItemCode::BI248 => Some(&ITEM_BI248),
        ItemCode::BI249 => Some(&ITEM_BI249),
        ItemCode::BI250 => Some(&ITEM_BI250),
        ItemCode::BI251 => Some(&ITEM_BI251),
        ItemCode::BI252 => Some(&ITEM_BI252),
        ItemCode::BI253 => Some(&ITEM_BI253),
        ItemCode::BI254 => Some(&ITEM_BI254),
        ItemCode::BI255 => Some(&ITEM_BI255),
        ItemCode::BI256 => Some(&ITEM_BI256),
        ItemCode::BI257 => Some(&ITEM_BI257),
        ItemCode::BI258 => Some(&ITEM_BI258),
        ItemCode::BI259 => Some(&ITEM_BI259),
        ItemCode::BI260 => Some(&ITEM_BI260),
        ItemCode::BI261 => Some(&ITEM_BI261),
        ItemCode::BI262 => Some(&ITEM_BI262),
        ItemCode::BI266 => Some(&ITEM_BI266),
        ItemCode::BI267 => Some(&ITEM_BI267),
        ItemCode::BI268 => Some(&ITEM_BI268),
        ItemCode::BI269 => Some(&ITEM_BI269),
        ItemCode::BI270 => Some(&ITEM_BI270),
        ItemCode::RR101 => Some(&ITEM_RR101),
        ItemCode::RR102 => Some(&ITEM_RR102),
        ItemCode::RR103 => Some(&ITEM_RR103),
        ItemCode::RR104 => Some(&ITEM_RR104),
        ItemCode::RR105 => Some(&ITEM_RR105),
        ItemCode::RR106 => Some(&ITEM_RR106),
        ItemCode::RR107 => Some(&ITEM_RR107),
        ItemCode::RR108 => Some(&ITEM_RR108),
        ItemCode::RR109 => Some(&ITEM_RR109),
        ItemCode::RR110 => Some(&ITEM_RR110),
        ItemCode::RR111 => Some(&ITEM_RR111),
        ItemCode::RR112 => Some(&ITEM_RR112),
        ItemCode::EV101 => Some(&ITEM_EV101),
        ItemCode::EV102 => Some(&ITEM_EV102),
        ItemCode::EV103 => Some(&ITEM_EV103),
        ItemCode::EV104 => Some(&ITEM_EV104),
        ItemCode::EV105 => Some(&ITEM_EV105),
        ItemCode::EV106 => Some(&ITEM_EV106),
        ItemCode::EV107 => Some(&ITEM_EV107),
        ItemCode::EV108 => Some(&ITEM_EV108),
        ItemCode::EV201 => Some(&ITEM_EV201),
        ItemCode::EV202 => Some(&ITEM_EV202),
        ItemCode::PZ101 => Some(&ITEM_PZ101),
        ItemCode::PZ102 => Some(&ITEM_PZ102),
        ItemCode::PZ103 => Some(&ITEM_PZ103),
        ItemCode::PZ104 => Some(&ITEM_PZ104),
        ItemCode::PZ105 => Some(&ITEM_PZ105),
        ItemCode::PZ106 => Some(&ITEM_PZ106),
        ItemCode::PZ107 => Some(&ITEM_PZ107),
        ItemCode::PZ108 => Some(&ITEM_PZ108),
        ItemCode::PZ201 => Some(&ITEM_PZ201),
        ItemCode::PZ202 => Some(&ITEM_PZ202),
        ItemCode::PZ203 => Some(&ITEM_PZ203),
        ItemCode::PZ204 => Some(&ITEM_PZ204),
        ItemCode::PZ205 => Some(&ITEM_PZ205),
        ItemCode::PZ206 => Some(&ITEM_PZ206),
        ItemCode::PZ207 => Some(&ITEM_PZ207),
        ItemCode::PZ208 => Some(&ITEM_PZ208),
        ItemCode::CH101 => Some(&ITEM_CH101),
        ItemCode::CH102 => Some(&ITEM_CH102),
        ItemCode::CH103 => Some(&ITEM_CH103),
        ItemCode::CH104 => Some(&ITEM_CH104),
        ItemCode::SE101 => Some(&ITEM_SE101),
        ItemCode::SE102 => Some(&ITEM_SE102),
        ItemCode::SE103 => Some(&ITEM_SE103),
        ItemCode::SE104 => Some(&ITEM_SE104),
        ItemCode::SE105 => Some(&ITEM_SE105),
        ItemCode::PR101 => Some(&ITEM_PR101),
        ItemCode::PR102 => Some(&ITEM_PR102),
        ItemCode::PR103 => Some(&ITEM_PR103),
        ItemCode::PR104 => Some(&ITEM_PR104),
        ItemCode::PR105 => Some(&ITEM_PR105),
        ItemCode::PR106 => Some(&ITEM_PR106),
        ItemCode::PR107 => Some(&ITEM_PR107),
        ItemCode::PR108 => Some(&ITEM_PR108),
        ItemCode::PR120 => Some(&ITEM_PR120),
        ItemCode::PR121 => Some(&ITEM_PR121),
        ItemCode::PR122 => Some(&ITEM_PR122),
        ItemCode::PR123 => Some(&ITEM_PR123),
        ItemCode::PR124 => Some(&ITEM_PR124),
        ItemCode::PR125 => Some(&ITEM_PR125),
        ItemCode::PR126 => Some(&ITEM_PR126),
        ItemCode::PR127 => Some(&ITEM_PR127),
        ItemCode::PR128 => Some(&ITEM_PR128),
        ItemCode::PR129 => Some(&ITEM_PR129),
        ItemCode::PR130 => Some(&ITEM_PR130),
        ItemCode::PR201 => Some(&ITEM_PR201),
        ItemCode::PR202 => Some(&ITEM_PR202),
        ItemCode::PR203 => Some(&ITEM_PR203),
        ItemCode::PR204 => Some(&ITEM_PR204),
        ItemCode::PR205 => Some(&ITEM_PR205),
        ItemCode::PR206 => Some(&ITEM_PR206),
        ItemCode::PR207 => Some(&ITEM_PR207),
        ItemCode::PR208 => Some(&ITEM_PR208),
        ItemCode::PR209 => Some(&ITEM_PR209),
        ItemCode::PR210 => Some(&ITEM_PR210),
        ItemCode::PR211 => Some(&ITEM_PR211),
        ItemCode::PR212 => Some(&ITEM_PR212),
        ItemCode::PR213 => Some(&ITEM_PR213),
        ItemCode::PR214 => Some(&ITEM_PR214),
        ItemCode::PR301 => Some(&ITEM_PR301),
        ItemCode::PR302 => Some(&ITEM_PR302),
        ItemCode::PR303 => Some(&ITEM_PR303),
        ItemCode::PR304 => Some(&ITEM_PR304),
        ItemCode::PR305 => Some(&ITEM_PR305),
        ItemCode::PR306 => Some(&ITEM_PR306),
        ItemCode::PR307 => Some(&ITEM_PR307),
        ItemCode::PR308 => Some(&ITEM_PR308),
        ItemCode::PR309 => Some(&ITEM_PR309),
        ItemCode::PR310 => Some(&ITEM_PR310),
        ItemCode::PR311 => Some(&ITEM_PR311),
        ItemCode::PR312 => Some(&ITEM_PR312),
        ItemCode::PR313 => Some(&ITEM_PR313),
        ItemCode::PR314 => Some(&ITEM_PR314),
        ItemCode::PR401 => Some(&ITEM_PR401),
        ItemCode::PR402 => Some(&ITEM_PR402),
        ItemCode::PR403 => Some(&ITEM_PR403),
        ItemCode::PR404 => Some(&ITEM_PR404),
        ItemCode::PR405 => Some(&ITEM_PR405),
        ItemCode::PR406 => Some(&ITEM_PR406),
        ItemCode::PR407 => Some(&ITEM_PR407),
        ItemCode::PR408 => Some(&ITEM_PR408),
        ItemCode::PR409 => Some(&ITEM_PR409),
        ItemCode::PR410 => Some(&ITEM_PR410),
        ItemCode::PR411 => Some(&ITEM_PR411),
        ItemCode::PR412 => Some(&ITEM_PR412),
        ItemCode::PR413 => Some(&ITEM_PR413),
        ItemCode::PR414 => Some(&ITEM_PR414),
        ItemCode::WR101 => Some(&ITEM_WR101),
        ItemCode::WR102 => Some(&ITEM_WR102),
        ItemCode::WR103 => Some(&ITEM_WR103),
        ItemCode::WR104 => Some(&ITEM_WR104),
        ItemCode::WR105 => Some(&ITEM_WR105),
        ItemCode::WR106 => Some(&ITEM_WR106),
        ItemCode::WR107 => Some(&ITEM_WR107),
        ItemCode::WR108 => Some(&ITEM_WR108),
        ItemCode::WR109 => Some(&ITEM_WR109),
        ItemCode::DR101 => Some(&ITEM_DR101),
        ItemCode::DR102 => Some(&ITEM_DR102),
        ItemCode::DR103 => Some(&ITEM_DR103),
        ItemCode::DR104 => Some(&ITEM_DR104),
        ItemCode::DR105 => Some(&ITEM_DR105),
        ItemCode::DR106 => Some(&ITEM_DR106),
        ItemCode::DR107 => Some(&ITEM_DR107),
        ItemCode::DR108 => Some(&ITEM_DR108),
        ItemCode::DR109 => Some(&ITEM_DR109),
        ItemCode::DR110 => Some(&ITEM_DR110),
        ItemCode::DR111 => Some(&ITEM_DR111),
        ItemCode::CA131 => Some(&ITEM_CA131),
        ItemCode::CA133 => Some(&ITEM_CA133),
        ItemCode::CA135 => Some(&ITEM_CA135),
        ItemCode::CA137 => Some(&ITEM_CA137),
        ItemCode::CA143 => Some(&ITEM_CA143),
        ItemCode::CA145 => Some(&ITEM_CA145),
        ItemCode::CA154 => Some(&ITEM_CA154),
        ItemCode::CA155 => Some(&ITEM_CA155),
        ItemCode::CA166 => Some(&ITEM_CA166),
        ItemCode::CA167 => Some(&ITEM_CA167),
        ItemCode::CA168 => Some(&ITEM_CA168),
        ItemCode::CA169 => Some(&ITEM_CA169),
        ItemCode::CA185 => Some(&ITEM_CA185),
        ItemCode::CA186 => Some(&ITEM_CA186),
        ItemCode::CA187 => Some(&ITEM_CA187),
        ItemCode::CA188 => Some(&ITEM_CA188),
        ItemCode::CA189 => Some(&ITEM_CA189),
        ItemCode::CA190 => Some(&ITEM_CA190),
        ItemCode::CA191 => Some(&ITEM_CA191),
        ItemCode::CA192 => Some(&ITEM_CA192),
        ItemCode::CA193 => Some(&ITEM_CA193),
        ItemCode::CA194 => Some(&ITEM_CA194),
        ItemCode::CA195 => Some(&ITEM_CA195),
        ItemCode::CA196 => Some(&ITEM_CA196),
        ItemCode::CA232 => Some(&ITEM_CA232),
        ItemCode::CA234 => Some(&ITEM_CA234),
        ItemCode::CA235 => Some(&ITEM_CA235),
        ItemCode::CA237 => Some(&ITEM_CA237),
        ItemCode::CA243 => Some(&ITEM_CA243),
        ItemCode::CA245 => Some(&ITEM_CA245),
        ItemCode::CA254 => Some(&ITEM_CA254),
        ItemCode::CA255 => Some(&ITEM_CA255),
        ItemCode::CA266 => Some(&ITEM_CA266),
        ItemCode::CA267 => Some(&ITEM_CA267),
        ItemCode::CA268 => Some(&ITEM_CA268),
        ItemCode::CA269 => Some(&ITEM_CA269),
        ItemCode::CA285 => Some(&ITEM_CA285),
        ItemCode::CA286 => Some(&ITEM_CA286),
        ItemCode::CA287 => Some(&ITEM_CA287),
        ItemCode::CA288 => Some(&ITEM_CA288),
        ItemCode::CA289 => Some(&ITEM_CA289),
        ItemCode::CA290 => Some(&ITEM_CA290),
        ItemCode::CA291 => Some(&ITEM_CA291),
        ItemCode::CA292 => Some(&ITEM_CA292),
        ItemCode::CA293 => Some(&ITEM_CA293),
        ItemCode::CA294 => Some(&ITEM_CA294),
        ItemCode::CA295 => Some(&ITEM_CA295),
        ItemCode::CA296 => Some(&ITEM_CA296),
        ItemCode::CA501 => Some(&ITEM_CA501),
        ItemCode::CA502 => Some(&ITEM_CA502),
        ItemCode::CA503 => Some(&ITEM_CA503),
        ItemCode::CA504 => Some(&ITEM_CA504),
        ItemCode::CA505 => Some(&ITEM_CA505),
        ItemCode::CA506 => Some(&ITEM_CA506),
        ItemCode::CA507 => Some(&ITEM_CA507),
        ItemCode::CA508 => Some(&ITEM_CA508),
        ItemCode::CA509 => Some(&ITEM_CA509),
        ItemCode::CA510 => Some(&ITEM_CA510),
        ItemCode::CA511 => Some(&ITEM_CA511),
        ItemCode::CA512 => Some(&ITEM_CA512),
        ItemCode::CA515 => Some(&ITEM_CA515),
        ItemCode::CA516 => Some(&ITEM_CA516),
        ItemCode::CA517 => Some(&ITEM_CA517),
        ItemCode::CA518 => Some(&ITEM_CA518),
        ItemCode::CA519 => Some(&ITEM_CA519),
        ItemCode::CA520 => Some(&ITEM_CA520),
        ItemCode::CA521 => Some(&ITEM_CA521),
        ItemCode::CA522 => Some(&ITEM_CA522),
        ItemCode::CA523 => Some(&ITEM_CA523),
        ItemCode::CA524 => Some(&ITEM_CA524),
        ItemCode::CA525 => Some(&ITEM_CA525),
        ItemCode::CA526 => Some(&ITEM_CA526),
        ItemCode::CA527 => Some(&ITEM_CA527),
        ItemCode::CA528 => Some(&ITEM_CA528),
        ItemCode::CA531 => Some(&ITEM_CA531),
        ItemCode::CA532 => Some(&ITEM_CA532),
        ItemCode::CA533 => Some(&ITEM_CA533),
        ItemCode::CA534 => Some(&ITEM_CA534),
        ItemCode::CA535 => Some(&ITEM_CA535),
        ItemCode::CA536 => Some(&ITEM_CA536),
        ItemCode::CA537 => Some(&ITEM_CA537),
        ItemCode::CA538 => Some(&ITEM_CA538),
        ItemCode::CA539 => Some(&ITEM_CA539),
        ItemCode::CA540 => Some(&ITEM_CA540),
        ItemCode::CA541 => Some(&ITEM_CA541),
        ItemCode::CA542 => Some(&ITEM_CA542),
        ItemCode::CA543 => Some(&ITEM_CA543),
        ItemCode::CA544 => Some(&ITEM_CA544),
        ItemCode::CA545 => Some(&ITEM_CA545),
        ItemCode::CA546 => Some(&ITEM_CA546),
        ItemCode::CA547 => Some(&ITEM_CA547),
        ItemCode::CA548 => Some(&ITEM_CA548),
        ItemCode::CA601 => Some(&ITEM_CA601),
        ItemCode::CA602 => Some(&ITEM_CA602),
        ItemCode::CA603 => Some(&ITEM_CA603),
        ItemCode::CA604 => Some(&ITEM_CA604),
        ItemCode::CA605 => Some(&ITEM_CA605),
        ItemCode::CA606 => Some(&ITEM_CA606),
        ItemCode::CA607 => Some(&ITEM_CA607),
        ItemCode::CA608 => Some(&ITEM_CA608),
        ItemCode::CA609 => Some(&ITEM_CA609),
        ItemCode::CA610 => Some(&ITEM_CA610),
        ItemCode::CA611 => Some(&ITEM_CA611),
        ItemCode::CA612 => Some(&ITEM_CA612),
        ItemCode::CA615 => Some(&ITEM_CA615),
        ItemCode::CA616 => Some(&ITEM_CA616),
        ItemCode::CA617 => Some(&ITEM_CA617),
        ItemCode::CA618 => Some(&ITEM_CA618),
        ItemCode::CA619 => Some(&ITEM_CA619),
        ItemCode::CA620 => Some(&ITEM_CA620),
        ItemCode::CA621 => Some(&ITEM_CA621),
        ItemCode::CA622 => Some(&ITEM_CA622),
        ItemCode::CA623 => Some(&ITEM_CA623),
        ItemCode::CA624 => Some(&ITEM_CA624),
        ItemCode::CA625 => Some(&ITEM_CA625),
        ItemCode::CA626 => Some(&ITEM_CA626),
        ItemCode::CA627 => Some(&ITEM_CA627),
        ItemCode::CA628 => Some(&ITEM_CA628),
        ItemCode::CA631 => Some(&ITEM_CA631),
        ItemCode::CA632 => Some(&ITEM_CA632),
        ItemCode::CA633 => Some(&ITEM_CA633),
        ItemCode::CA634 => Some(&ITEM_CA634),
        ItemCode::CA635 => Some(&ITEM_CA635),
        ItemCode::CA636 => Some(&ITEM_CA636),
        ItemCode::CA637 => Some(&ITEM_CA637),
        ItemCode::CA638 => Some(&ITEM_CA638),
        ItemCode::CA639 => Some(&ITEM_CA639),
        ItemCode::CA640 => Some(&ITEM_CA640),
        ItemCode::CA641 => Some(&ITEM_CA641),
        ItemCode::CA642 => Some(&ITEM_CA642),
        ItemCode::CA643 => Some(&ITEM_CA643),
        ItemCode::CA644 => Some(&ITEM_CA644),
        ItemCode::CA645 => Some(&ITEM_CA645),
        ItemCode::CA646 => Some(&ITEM_CA646),
        ItemCode::CA647 => Some(&ITEM_CA647),
        ItemCode::CA648 => Some(&ITEM_CA648),
        ItemCode::CA649 => Some(&ITEM_CA649),
        ItemCode::CA549 => Some(&ITEM_CA549),
        ItemCode::CA650 => Some(&ITEM_CA650),
        ItemCode::CA550 => Some(&ITEM_CA550),
        ItemCode::CA651 => Some(&ITEM_CA651),
        ItemCode::CA551 => Some(&ITEM_CA551),
        ItemCode::CA652 => Some(&ITEM_CA652),
        ItemCode::CA552 => Some(&ITEM_CA552),
        ItemCode::CA653 => Some(&ITEM_CA653),
        ItemCode::CA553 => Some(&ITEM_CA553),
        ItemCode::CA654 => Some(&ITEM_CA654),
        ItemCode::CA554 => Some(&ITEM_CA554),
        ItemCode::CA655 => Some(&ITEM_CA655),
        ItemCode::CA555 => Some(&ITEM_CA555),
        ItemCode::CA656 => Some(&ITEM_CA656),
        ItemCode::CA556 => Some(&ITEM_CA556),
        ItemCode::CA657 => Some(&ITEM_CA657),
        ItemCode::CA557 => Some(&ITEM_CA557),
        ItemCode::CA658 => Some(&ITEM_CA658),
        ItemCode::CA558 => Some(&ITEM_CA558),
        ItemCode::CA659 => Some(&ITEM_CA659),
        ItemCode::CA559 => Some(&ITEM_CA559),
        ItemCode::CA660 => Some(&ITEM_CA660),
        ItemCode::CA560 => Some(&ITEM_CA560),
        ItemCode::CA661 => Some(&ITEM_CA661),
        ItemCode::CA561 => Some(&ITEM_CA561),
        ItemCode::CA662 => Some(&ITEM_CA662),
        ItemCode::CA562 => Some(&ITEM_CA562),
        ItemCode::CA663 => Some(&ITEM_CA663),
        ItemCode::CA563 => Some(&ITEM_CA563),
        ItemCode::CA664 => Some(&ITEM_CA664),
        ItemCode::CA564 => Some(&ITEM_CA564),
        ItemCode::DB144 => Some(&ITEM_DB144),
        ItemCode::DG131 => Some(&ITEM_DG131),
        ItemCode::OA230 => Some(&ITEM_OA230),
        ItemCode::DA159 => Some(&ITEM_DA159),
        ItemCode::DA161 => Some(&ITEM_DA161),
        ItemCode::DA165 => Some(&ITEM_DA165),
        ItemCode::GG101 => Some(&ITEM_GG101),
        ItemCode::GG102 => Some(&ITEM_GG102),
        ItemCode::OS118 => Some(&ITEM_OS118),
        ItemCode::OS119 => Some(&ITEM_OS119),
        ItemCode::OS120 => Some(&ITEM_OS120),
        ItemCode::GG103 => Some(&ITEM_GG103),
        ItemCode::BI301 => Some(&ITEM_BI301),
        ItemCode::BI302 => Some(&ITEM_BI302),
        ItemCode::BI303 => Some(&ITEM_BI303),
        ItemCode::BI304 => Some(&ITEM_BI304),
        ItemCode::BI305 => Some(&ITEM_BI305),
        ItemCode::BI306 => Some(&ITEM_BI306),
        ItemCode::BI307 => Some(&ITEM_BI307),
        ItemCode::BI308 => Some(&ITEM_BI308),
        ItemCode::BI309 => Some(&ITEM_BI309),
        ItemCode::BI310 => Some(&ITEM_BI310),
        ItemCode::BI311 => Some(&ITEM_BI311),
        ItemCode::BI312 => Some(&ITEM_BI312),
        ItemCode::BI313 => Some(&ITEM_BI313),
        ItemCode::BI314 => Some(&ITEM_BI314),
        ItemCode::BI315 => Some(&ITEM_BI315),
        ItemCode::BI316 => Some(&ITEM_BI316),
        ItemCode::BI317 => Some(&ITEM_BI317),
        ItemCode::BI318 => Some(&ITEM_BI318),
        ItemCode::BI319 => Some(&ITEM_BI319),
        ItemCode::BI320 => Some(&ITEM_BI320),
        ItemCode::BI321 => Some(&ITEM_BI321),
        ItemCode::BI322 => Some(&ITEM_BI322),
        ItemCode::BI323 => Some(&ITEM_BI323),
        ItemCode::BI324 => Some(&ITEM_BI324),
        ItemCode::BI325 => Some(&ITEM_BI325),
        ItemCode::BI326 => Some(&ITEM_BI326),
        ItemCode::BI330 => Some(&ITEM_BI330),
        ItemCode::BI331 => Some(&ITEM_BI331),
        ItemCode::BI332 => Some(&ITEM_BI332),
        ItemCode::BI333 => Some(&ITEM_BI333),
        ItemCode::BI334 => Some(&ITEM_BI334),
        ItemCode::BI335 => Some(&ITEM_BI335),
        ItemCode::BI336 => Some(&ITEM_BI336),
        ItemCode::BI340 => Some(&ITEM_BI340),
        ItemCode::BI341 => Some(&ITEM_BI341),
        ItemCode::BI342 => Some(&ITEM_BI342),
        ItemCode::BI343 => Some(&ITEM_BI343),
        ItemCode::BI344 => Some(&ITEM_BI344),
        ItemCode::BI345 => Some(&ITEM_BI345),
        ItemCode::BI380 => Some(&ITEM_BI380),
        ItemCode::BI381 => Some(&ITEM_BI381),
        ItemCode::BI382 => Some(&ITEM_BI382),
        ItemCode::BI383 => Some(&ITEM_BI383),
        ItemCode::BI384 => Some(&ITEM_BI384),
        ItemCode::BI385 => Some(&ITEM_BI385),
        ItemCode::BI386 => Some(&ITEM_BI386),
        ItemCode::BI387 => Some(&ITEM_BI387),
        ItemCode::BI388 => Some(&ITEM_BI388),
        ItemCode::BI389 => Some(&ITEM_BI389),
        ItemCode::BI390 => Some(&ITEM_BI390),
        ItemCode::BI391 => Some(&ITEM_BI391),
        ItemCode::BI392 => Some(&ITEM_BI392),
        ItemCode::BI393 => Some(&ITEM_BI393),
        ItemCode::WA186 => Some(&ITEM_WA186),
        ItemCode::WA188 => Some(&ITEM_WA188),
        ItemCode::WC186 => Some(&ITEM_WC186),
        ItemCode::WH186 => Some(&ITEM_WH186),
        ItemCode::WH188 => Some(&ITEM_WH188),
        ItemCode::WP186 => Some(&ITEM_WP186),
        ItemCode::WS186 => Some(&ITEM_WS186),
        ItemCode::WS286 => Some(&ITEM_WS286),
        ItemCode::WS288 => Some(&ITEM_WS288),
        ItemCode::WM186 => Some(&ITEM_WM186),
        ItemCode::WM188 => Some(&ITEM_WM188),
        ItemCode::WD186 => Some(&ITEM_WD186),
        ItemCode::WN186 => Some(&ITEM_WN186),
        ItemCode::WV186 => Some(&ITEM_WV186),
        ItemCode::WT186 => Some(&ITEM_WT186),
        ItemCode::DG186 => Some(&ITEM_DG186),
        ItemCode::DB186 => Some(&ITEM_DB186),
        ItemCode::OA286 => Some(&ITEM_OA286),
        ItemCode::DS186 => Some(&ITEM_DS186),
        ItemCode::OE101 => Some(&ITEM_OE101),
        ItemCode::OE102 => Some(&ITEM_OE102),
        ItemCode::OE103 => Some(&ITEM_OE103),
        ItemCode::OE104 => Some(&ITEM_OE104),
        ItemCode::OE105 => Some(&ITEM_OE105),
        ItemCode::OE106 => Some(&ITEM_OE106),
        ItemCode::OA151 => Some(&ITEM_OA151),
        ItemCode::OA153 => Some(&ITEM_OA153),
        ItemCode::BI263 => Some(&ITEM_BI263),
        ItemCode::BI264 => Some(&ITEM_BI264),
        ItemCode::BI265 => Some(&ITEM_BI265),
        _ => None,
    }
}

static ITEM_WA101: ItemRow = ItemRow {
    code: ItemCode::WA101,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA102: ItemRow = ItemRow {
    code: ItemCode::WA102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA103: ItemRow = ItemRow {
    code: ItemCode::WA103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA104: ItemRow = ItemRow {
    code: ItemCode::WA104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA105: ItemRow = ItemRow {
    code: ItemCode::WA105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA106: ItemRow = ItemRow {
    code: ItemCode::WA106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA107: ItemRow = ItemRow {
    code: ItemCode::WA107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA108: ItemRow = ItemRow {
    code: ItemCode::WA108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA109: ItemRow = ItemRow {
    code: ItemCode::WA109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA110: ItemRow = ItemRow {
    code: ItemCode::WA110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA111: ItemRow = ItemRow {
    code: ItemCode::WA111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA112: ItemRow = ItemRow {
    code: ItemCode::WA112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA113: ItemRow = ItemRow {
    code: ItemCode::WA113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA114: ItemRow = ItemRow {
    code: ItemCode::WA114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA115: ItemRow = ItemRow {
    code: ItemCode::WA115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC101: ItemRow = ItemRow {
    code: ItemCode::WC101,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC102: ItemRow = ItemRow {
    code: ItemCode::WC102,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC103: ItemRow = ItemRow {
    code: ItemCode::WC103,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC104: ItemRow = ItemRow {
    code: ItemCode::WC104,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC105: ItemRow = ItemRow {
    code: ItemCode::WC105,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC106: ItemRow = ItemRow {
    code: ItemCode::WC106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC107: ItemRow = ItemRow {
    code: ItemCode::WC107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC108: ItemRow = ItemRow {
    code: ItemCode::WC108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC109: ItemRow = ItemRow {
    code: ItemCode::WC109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC110: ItemRow = ItemRow {
    code: ItemCode::WC110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC111: ItemRow = ItemRow {
    code: ItemCode::WC111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC112: ItemRow = ItemRow {
    code: ItemCode::WC112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC113: ItemRow = ItemRow {
    code: ItemCode::WC113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC114: ItemRow = ItemRow {
    code: ItemCode::WC114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC115: ItemRow = ItemRow {
    code: ItemCode::WC115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH101: ItemRow = ItemRow {
    code: ItemCode::WH101,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH102: ItemRow = ItemRow {
    code: ItemCode::WH102,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH103: ItemRow = ItemRow {
    code: ItemCode::WH103,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH104: ItemRow = ItemRow {
    code: ItemCode::WH104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH105: ItemRow = ItemRow {
    code: ItemCode::WH105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH106: ItemRow = ItemRow {
    code: ItemCode::WH106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH107: ItemRow = ItemRow {
    code: ItemCode::WH107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH108: ItemRow = ItemRow {
    code: ItemCode::WH108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH109: ItemRow = ItemRow {
    code: ItemCode::WH109,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH110: ItemRow = ItemRow {
    code: ItemCode::WH110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH111: ItemRow = ItemRow {
    code: ItemCode::WH111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH112: ItemRow = ItemRow {
    code: ItemCode::WH112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH113: ItemRow = ItemRow {
    code: ItemCode::WH113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH114: ItemRow = ItemRow {
    code: ItemCode::WH114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH115: ItemRow = ItemRow {
    code: ItemCode::WH115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH116: ItemRow = ItemRow {
    code: ItemCode::WH116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WM101: ItemRow = ItemRow {
    code: ItemCode::WM101,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM102: ItemRow = ItemRow {
    code: ItemCode::WM102,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM103: ItemRow = ItemRow {
    code: ItemCode::WM103,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM104: ItemRow = ItemRow {
    code: ItemCode::WM104,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM105: ItemRow = ItemRow {
    code: ItemCode::WM105,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM106: ItemRow = ItemRow {
    code: ItemCode::WM106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM107: ItemRow = ItemRow {
    code: ItemCode::WM107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM108: ItemRow = ItemRow {
    code: ItemCode::WM108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM109: ItemRow = ItemRow {
    code: ItemCode::WM109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM110: ItemRow = ItemRow {
    code: ItemCode::WM110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM111: ItemRow = ItemRow {
    code: ItemCode::WM111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM112: ItemRow = ItemRow {
    code: ItemCode::WM112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM113: ItemRow = ItemRow {
    code: ItemCode::WM113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM114: ItemRow = ItemRow {
    code: ItemCode::WM114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM115: ItemRow = ItemRow {
    code: ItemCode::WM115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM116: ItemRow = ItemRow {
    code: ItemCode::WM116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WP101: ItemRow = ItemRow {
    code: ItemCode::WP101,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP102: ItemRow = ItemRow {
    code: ItemCode::WP102,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP103: ItemRow = ItemRow {
    code: ItemCode::WP103,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP104: ItemRow = ItemRow {
    code: ItemCode::WP104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP105: ItemRow = ItemRow {
    code: ItemCode::WP105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP106: ItemRow = ItemRow {
    code: ItemCode::WP106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP107: ItemRow = ItemRow {
    code: ItemCode::WP107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP108: ItemRow = ItemRow {
    code: ItemCode::WP108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP109: ItemRow = ItemRow {
    code: ItemCode::WP109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP110: ItemRow = ItemRow {
    code: ItemCode::WP110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP111: ItemRow = ItemRow {
    code: ItemCode::WP111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP112: ItemRow = ItemRow {
    code: ItemCode::WP112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP113: ItemRow = ItemRow {
    code: ItemCode::WP113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP114: ItemRow = ItemRow {
    code: ItemCode::WP114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP115: ItemRow = ItemRow {
    code: ItemCode::WP115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP116: ItemRow = ItemRow {
    code: ItemCode::WP116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS101: ItemRow = ItemRow {
    code: ItemCode::WS101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS102: ItemRow = ItemRow {
    code: ItemCode::WS102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS103: ItemRow = ItemRow {
    code: ItemCode::WS103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS104: ItemRow = ItemRow {
    code: ItemCode::WS104,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS105: ItemRow = ItemRow {
    code: ItemCode::WS105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS106: ItemRow = ItemRow {
    code: ItemCode::WS106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS107: ItemRow = ItemRow {
    code: ItemCode::WS107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS108: ItemRow = ItemRow {
    code: ItemCode::WS108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS109: ItemRow = ItemRow {
    code: ItemCode::WS109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS110: ItemRow = ItemRow {
    code: ItemCode::WS110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS111: ItemRow = ItemRow {
    code: ItemCode::WS111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS112: ItemRow = ItemRow {
    code: ItemCode::WS112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS113: ItemRow = ItemRow {
    code: ItemCode::WS113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS114: ItemRow = ItemRow {
    code: ItemCode::WS114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS115: ItemRow = ItemRow {
    code: ItemCode::WS115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS116: ItemRow = ItemRow {
    code: ItemCode::WS116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS117: ItemRow = ItemRow {
    code: ItemCode::WS117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS201: ItemRow = ItemRow {
    code: ItemCode::WS201,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS202: ItemRow = ItemRow {
    code: ItemCode::WS202,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS203: ItemRow = ItemRow {
    code: ItemCode::WS203,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS204: ItemRow = ItemRow {
    code: ItemCode::WS204,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS205: ItemRow = ItemRow {
    code: ItemCode::WS205,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS206: ItemRow = ItemRow {
    code: ItemCode::WS206,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS207: ItemRow = ItemRow {
    code: ItemCode::WS207,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS208: ItemRow = ItemRow {
    code: ItemCode::WS208,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS209: ItemRow = ItemRow {
    code: ItemCode::WS209,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS210: ItemRow = ItemRow {
    code: ItemCode::WS210,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS211: ItemRow = ItemRow {
    code: ItemCode::WS211,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS212: ItemRow = ItemRow {
    code: ItemCode::WS212,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS213: ItemRow = ItemRow {
    code: ItemCode::WS213,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS214: ItemRow = ItemRow {
    code: ItemCode::WS214,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS215: ItemRow = ItemRow {
    code: ItemCode::WS215,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS216: ItemRow = ItemRow {
    code: ItemCode::WS216,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS217: ItemRow = ItemRow {
    code: ItemCode::WS217,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS218: ItemRow = ItemRow {
    code: ItemCode::WS218,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WT101: ItemRow = ItemRow {
    code: ItemCode::WT101,
    w: ITEMSIZE,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT102: ItemRow = ItemRow {
    code: ItemCode::WT102,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT103: ItemRow = ItemRow {
    code: ItemCode::WT103,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT104: ItemRow = ItemRow {
    code: ItemCode::WT104,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT105: ItemRow = ItemRow {
    code: ItemCode::WT105,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT106: ItemRow = ItemRow {
    code: ItemCode::WT106,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT107: ItemRow = ItemRow {
    code: ItemCode::WT107,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT108: ItemRow = ItemRow {
    code: ItemCode::WT108,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT109: ItemRow = ItemRow {
    code: ItemCode::WT109,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT110: ItemRow = ItemRow {
    code: ItemCode::WT110,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT111: ItemRow = ItemRow {
    code: ItemCode::WT111,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT112: ItemRow = ItemRow {
    code: ItemCode::WT112,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT113: ItemRow = ItemRow {
    code: ItemCode::WT113,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT114: ItemRow = ItemRow {
    code: ItemCode::WT114,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT115: ItemRow = ItemRow {
    code: ItemCode::WT115,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT116: ItemRow = ItemRow {
    code: ItemCode::WT116,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WA116: ItemRow = ItemRow {
    code: ItemCode::WA116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA117: ItemRow = ItemRow {
    code: ItemCode::WA117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA118: ItemRow = ItemRow {
    code: ItemCode::WA118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA119: ItemRow = ItemRow {
    code: ItemCode::WA119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA120: ItemRow = ItemRow {
    code: ItemCode::WA120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA121: ItemRow = ItemRow {
    code: ItemCode::WA121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA122: ItemRow = ItemRow {
    code: ItemCode::WA122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA123: ItemRow = ItemRow {
    code: ItemCode::WA123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA124: ItemRow = ItemRow {
    code: ItemCode::WA124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA125: ItemRow = ItemRow {
    code: ItemCode::WA125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA126: ItemRow = ItemRow {
    code: ItemCode::WA126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA127: ItemRow = ItemRow {
    code: ItemCode::WA127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA128: ItemRow = ItemRow {
    code: ItemCode::WA128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA129: ItemRow = ItemRow {
    code: ItemCode::WA129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA130: ItemRow = ItemRow {
    code: ItemCode::WA130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA131: ItemRow = ItemRow {
    code: ItemCode::WA131,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC116: ItemRow = ItemRow {
    code: ItemCode::WC116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC117: ItemRow = ItemRow {
    code: ItemCode::WC117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC118: ItemRow = ItemRow {
    code: ItemCode::WC118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC119: ItemRow = ItemRow {
    code: ItemCode::WC119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC120: ItemRow = ItemRow {
    code: ItemCode::WC120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC121: ItemRow = ItemRow {
    code: ItemCode::WC121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC122: ItemRow = ItemRow {
    code: ItemCode::WC122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC123: ItemRow = ItemRow {
    code: ItemCode::WC123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC124: ItemRow = ItemRow {
    code: ItemCode::WC124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC125: ItemRow = ItemRow {
    code: ItemCode::WC125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC126: ItemRow = ItemRow {
    code: ItemCode::WC126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC127: ItemRow = ItemRow {
    code: ItemCode::WC127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC128: ItemRow = ItemRow {
    code: ItemCode::WC128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC129: ItemRow = ItemRow {
    code: ItemCode::WC129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC130: ItemRow = ItemRow {
    code: ItemCode::WC130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH117: ItemRow = ItemRow {
    code: ItemCode::WH117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH118: ItemRow = ItemRow {
    code: ItemCode::WH118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH119: ItemRow = ItemRow {
    code: ItemCode::WH119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH120: ItemRow = ItemRow {
    code: ItemCode::WH120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH121: ItemRow = ItemRow {
    code: ItemCode::WH121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH122: ItemRow = ItemRow {
    code: ItemCode::WH122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH123: ItemRow = ItemRow {
    code: ItemCode::WH123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH124: ItemRow = ItemRow {
    code: ItemCode::WH124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH125: ItemRow = ItemRow {
    code: ItemCode::WH125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH126: ItemRow = ItemRow {
    code: ItemCode::WH126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH127: ItemRow = ItemRow {
    code: ItemCode::WH127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH128: ItemRow = ItemRow {
    code: ItemCode::WH128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH129: ItemRow = ItemRow {
    code: ItemCode::WH129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH130: ItemRow = ItemRow {
    code: ItemCode::WH130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH131: ItemRow = ItemRow {
    code: ItemCode::WH131,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH132: ItemRow = ItemRow {
    code: ItemCode::WH132,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WM117: ItemRow = ItemRow {
    code: ItemCode::WM117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM118: ItemRow = ItemRow {
    code: ItemCode::WM118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM119: ItemRow = ItemRow {
    code: ItemCode::WM119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM120: ItemRow = ItemRow {
    code: ItemCode::WM120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM121: ItemRow = ItemRow {
    code: ItemCode::WM121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM122: ItemRow = ItemRow {
    code: ItemCode::WM122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM123: ItemRow = ItemRow {
    code: ItemCode::WM123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM124: ItemRow = ItemRow {
    code: ItemCode::WM124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM125: ItemRow = ItemRow {
    code: ItemCode::WM125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM126: ItemRow = ItemRow {
    code: ItemCode::WM126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM127: ItemRow = ItemRow {
    code: ItemCode::WM127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM128: ItemRow = ItemRow {
    code: ItemCode::WM128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM129: ItemRow = ItemRow {
    code: ItemCode::WM129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM130: ItemRow = ItemRow {
    code: ItemCode::WM130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM131: ItemRow = ItemRow {
    code: ItemCode::WM131,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM132: ItemRow = ItemRow {
    code: ItemCode::WM132,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WP117: ItemRow = ItemRow {
    code: ItemCode::WP117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP118: ItemRow = ItemRow {
    code: ItemCode::WP118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP119: ItemRow = ItemRow {
    code: ItemCode::WP119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP120: ItemRow = ItemRow {
    code: ItemCode::WP120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP121: ItemRow = ItemRow {
    code: ItemCode::WP121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP122: ItemRow = ItemRow {
    code: ItemCode::WP122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP123: ItemRow = ItemRow {
    code: ItemCode::WP123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP124: ItemRow = ItemRow {
    code: ItemCode::WP124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP125: ItemRow = ItemRow {
    code: ItemCode::WP125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP126: ItemRow = ItemRow {
    code: ItemCode::WP126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP127: ItemRow = ItemRow {
    code: ItemCode::WP127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP128: ItemRow = ItemRow {
    code: ItemCode::WP128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP129: ItemRow = ItemRow {
    code: ItemCode::WP129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP130: ItemRow = ItemRow {
    code: ItemCode::WP130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP131: ItemRow = ItemRow {
    code: ItemCode::WP131,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP132: ItemRow = ItemRow {
    code: ItemCode::WP132,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP133: ItemRow = ItemRow {
    code: ItemCode::WP133,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP134: ItemRow = ItemRow {
    code: ItemCode::WP134,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP135: ItemRow = ItemRow {
    code: ItemCode::WP135,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP136: ItemRow = ItemRow {
    code: ItemCode::WP136,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS118: ItemRow = ItemRow {
    code: ItemCode::WS118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS119: ItemRow = ItemRow {
    code: ItemCode::WS119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS120: ItemRow = ItemRow {
    code: ItemCode::WS120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS121: ItemRow = ItemRow {
    code: ItemCode::WS121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS122: ItemRow = ItemRow {
    code: ItemCode::WS122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS123: ItemRow = ItemRow {
    code: ItemCode::WS123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS124: ItemRow = ItemRow {
    code: ItemCode::WS124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS125: ItemRow = ItemRow {
    code: ItemCode::WS125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS126: ItemRow = ItemRow {
    code: ItemCode::WS126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS127: ItemRow = ItemRow {
    code: ItemCode::WS127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS128: ItemRow = ItemRow {
    code: ItemCode::WS128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS129: ItemRow = ItemRow {
    code: ItemCode::WS129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS130: ItemRow = ItemRow {
    code: ItemCode::WS130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS131: ItemRow = ItemRow {
    code: ItemCode::WS131,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS132: ItemRow = ItemRow {
    code: ItemCode::WS132,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS219: ItemRow = ItemRow {
    code: ItemCode::WS219,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS220: ItemRow = ItemRow {
    code: ItemCode::WS220,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS221: ItemRow = ItemRow {
    code: ItemCode::WS221,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS222: ItemRow = ItemRow {
    code: ItemCode::WS222,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS223: ItemRow = ItemRow {
    code: ItemCode::WS223,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS224: ItemRow = ItemRow {
    code: ItemCode::WS224,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS225: ItemRow = ItemRow {
    code: ItemCode::WS225,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS226: ItemRow = ItemRow {
    code: ItemCode::WS226,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS227: ItemRow = ItemRow {
    code: ItemCode::WS227,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS228: ItemRow = ItemRow {
    code: ItemCode::WS228,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS229: ItemRow = ItemRow {
    code: ItemCode::WS229,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS230: ItemRow = ItemRow {
    code: ItemCode::WS230,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS231: ItemRow = ItemRow {
    code: ItemCode::WS231,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS232: ItemRow = ItemRow {
    code: ItemCode::WS232,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS233: ItemRow = ItemRow {
    code: ItemCode::WS233,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS234: ItemRow = ItemRow {
    code: ItemCode::WS234,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS251: ItemRow = ItemRow {
    code: ItemCode::WS251,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WT117: ItemRow = ItemRow {
    code: ItemCode::WT117,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT118: ItemRow = ItemRow {
    code: ItemCode::WT118,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT119: ItemRow = ItemRow {
    code: ItemCode::WT119,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT120: ItemRow = ItemRow {
    code: ItemCode::WT120,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT121: ItemRow = ItemRow {
    code: ItemCode::WT121,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT122: ItemRow = ItemRow {
    code: ItemCode::WT122,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT123: ItemRow = ItemRow {
    code: ItemCode::WT123,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT124: ItemRow = ItemRow {
    code: ItemCode::WT124,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT125: ItemRow = ItemRow {
    code: ItemCode::WT125,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT126: ItemRow = ItemRow {
    code: ItemCode::WT126,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT127: ItemRow = ItemRow {
    code: ItemCode::WT127,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT128: ItemRow = ItemRow {
    code: ItemCode::WT128,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT129: ItemRow = ItemRow {
    code: ItemCode::WT129,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT130: ItemRow = ItemRow {
    code: ItemCode::WT130,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WT131: ItemRow = ItemRow {
    code: ItemCode::WT131,
    w: ITEMSIZE,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_OA101: ItemRow = ItemRow {
    code: ItemCode::OA101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA102: ItemRow = ItemRow {
    code: ItemCode::OA102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA103: ItemRow = ItemRow {
    code: ItemCode::OA103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA104: ItemRow = ItemRow {
    code: ItemCode::OA104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA105: ItemRow = ItemRow {
    code: ItemCode::OA105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA106: ItemRow = ItemRow {
    code: ItemCode::OA106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA107: ItemRow = ItemRow {
    code: ItemCode::OA107,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA108: ItemRow = ItemRow {
    code: ItemCode::OA108,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA109: ItemRow = ItemRow {
    code: ItemCode::OA109,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA110: ItemRow = ItemRow {
    code: ItemCode::OA110,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA111: ItemRow = ItemRow {
    code: ItemCode::OA111,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA112: ItemRow = ItemRow {
    code: ItemCode::OA112,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA113: ItemRow = ItemRow {
    code: ItemCode::OA113,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA114: ItemRow = ItemRow {
    code: ItemCode::OA114,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA115: ItemRow = ItemRow {
    code: ItemCode::OA115,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA116: ItemRow = ItemRow {
    code: ItemCode::OA116,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA117: ItemRow = ItemRow {
    code: ItemCode::OA117,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA118: ItemRow = ItemRow {
    code: ItemCode::OA118,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA119: ItemRow = ItemRow {
    code: ItemCode::OA119,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA120: ItemRow = ItemRow {
    code: ItemCode::OA120,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA121: ItemRow = ItemRow {
    code: ItemCode::OA121,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA122: ItemRow = ItemRow {
    code: ItemCode::OA122,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA123: ItemRow = ItemRow {
    code: ItemCode::OA123,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA124: ItemRow = ItemRow {
    code: ItemCode::OA124,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA125: ItemRow = ItemRow {
    code: ItemCode::OA125,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA126: ItemRow = ItemRow {
    code: ItemCode::OA126,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA129: ItemRow = ItemRow {
    code: ItemCode::OA129,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA130: ItemRow = ItemRow {
    code: ItemCode::OA130,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA131: ItemRow = ItemRow {
    code: ItemCode::OA131,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA132: ItemRow = ItemRow {
    code: ItemCode::OA132,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA133: ItemRow = ItemRow {
    code: ItemCode::OA133,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA134: ItemRow = ItemRow {
    code: ItemCode::OA134,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA135: ItemRow = ItemRow {
    code: ItemCode::OA135,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA136: ItemRow = ItemRow {
    code: ItemCode::OA136,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA137: ItemRow = ItemRow {
    code: ItemCode::OA137,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA138: ItemRow = ItemRow {
    code: ItemCode::OA138,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA139: ItemRow = ItemRow {
    code: ItemCode::OA139,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA140: ItemRow = ItemRow {
    code: ItemCode::OA140,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA141: ItemRow = ItemRow {
    code: ItemCode::OA141,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA142: ItemRow = ItemRow {
    code: ItemCode::OA142,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA201: ItemRow = ItemRow {
    code: ItemCode::OA201,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA202: ItemRow = ItemRow {
    code: ItemCode::OA202,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WN101: ItemRow = ItemRow {
    code: ItemCode::WN101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN102: ItemRow = ItemRow {
    code: ItemCode::WN102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN103: ItemRow = ItemRow {
    code: ItemCode::WN103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN104: ItemRow = ItemRow {
    code: ItemCode::WN104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN105: ItemRow = ItemRow {
    code: ItemCode::WN105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN106: ItemRow = ItemRow {
    code: ItemCode::WN106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN107: ItemRow = ItemRow {
    code: ItemCode::WN107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN108: ItemRow = ItemRow {
    code: ItemCode::WN108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN109: ItemRow = ItemRow {
    code: ItemCode::WN109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN110: ItemRow = ItemRow {
    code: ItemCode::WN110,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN111: ItemRow = ItemRow {
    code: ItemCode::WN111,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN112: ItemRow = ItemRow {
    code: ItemCode::WN112,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN113: ItemRow = ItemRow {
    code: ItemCode::WN113,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN114: ItemRow = ItemRow {
    code: ItemCode::WN114,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN115: ItemRow = ItemRow {
    code: ItemCode::WN115,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN116: ItemRow = ItemRow {
    code: ItemCode::WN116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN117: ItemRow = ItemRow {
    code: ItemCode::WN117,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN118: ItemRow = ItemRow {
    code: ItemCode::WN118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN119: ItemRow = ItemRow {
    code: ItemCode::WN119,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN120: ItemRow = ItemRow {
    code: ItemCode::WN120,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN121: ItemRow = ItemRow {
    code: ItemCode::WN121,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN122: ItemRow = ItemRow {
    code: ItemCode::WN122,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN123: ItemRow = ItemRow {
    code: ItemCode::WN123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN124: ItemRow = ItemRow {
    code: ItemCode::WN124,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN125: ItemRow = ItemRow {
    code: ItemCode::WN125,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN126: ItemRow = ItemRow {
    code: ItemCode::WN126,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN127: ItemRow = ItemRow {
    code: ItemCode::WN127,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN128: ItemRow = ItemRow {
    code: ItemCode::WN128,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN129: ItemRow = ItemRow {
    code: ItemCode::WN129,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN130: ItemRow = ItemRow {
    code: ItemCode::WN130,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN152: ItemRow = ItemRow {
    code: ItemCode::WN152,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN153: ItemRow = ItemRow {
    code: ItemCode::WN153,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN154: ItemRow = ItemRow {
    code: ItemCode::WN154,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN155: ItemRow = ItemRow {
    code: ItemCode::WN155,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WN156: ItemRow = ItemRow {
    code: ItemCode::WN156,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WD101: ItemRow = ItemRow {
    code: ItemCode::WD101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD102: ItemRow = ItemRow {
    code: ItemCode::WD102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD103: ItemRow = ItemRow {
    code: ItemCode::WD103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD104: ItemRow = ItemRow {
    code: ItemCode::WD104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD105: ItemRow = ItemRow {
    code: ItemCode::WD105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD106: ItemRow = ItemRow {
    code: ItemCode::WD106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD107: ItemRow = ItemRow {
    code: ItemCode::WD107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD108: ItemRow = ItemRow {
    code: ItemCode::WD108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD109: ItemRow = ItemRow {
    code: ItemCode::WD109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD110: ItemRow = ItemRow {
    code: ItemCode::WD110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD111: ItemRow = ItemRow {
    code: ItemCode::WD111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD112: ItemRow = ItemRow {
    code: ItemCode::WD112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD113: ItemRow = ItemRow {
    code: ItemCode::WD113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD114: ItemRow = ItemRow {
    code: ItemCode::WD114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD115: ItemRow = ItemRow {
    code: ItemCode::WD115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD116: ItemRow = ItemRow {
    code: ItemCode::WD116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD117: ItemRow = ItemRow {
    code: ItemCode::WD117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD118: ItemRow = ItemRow {
    code: ItemCode::WD118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD119: ItemRow = ItemRow {
    code: ItemCode::WD119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD120: ItemRow = ItemRow {
    code: ItemCode::WD120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD121: ItemRow = ItemRow {
    code: ItemCode::WD121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD122: ItemRow = ItemRow {
    code: ItemCode::WD122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD123: ItemRow = ItemRow {
    code: ItemCode::WD123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD124: ItemRow = ItemRow {
    code: ItemCode::WD124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD125: ItemRow = ItemRow {
    code: ItemCode::WD125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD126: ItemRow = ItemRow {
    code: ItemCode::WD126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD127: ItemRow = ItemRow {
    code: ItemCode::WD127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD128: ItemRow = ItemRow {
    code: ItemCode::WD128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD129: ItemRow = ItemRow {
    code: ItemCode::WD129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD130: ItemRow = ItemRow {
    code: ItemCode::WD130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD152: ItemRow = ItemRow {
    code: ItemCode::WD152,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD153: ItemRow = ItemRow {
    code: ItemCode::WD153,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD154: ItemRow = ItemRow {
    code: ItemCode::WD154,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD155: ItemRow = ItemRow {
    code: ItemCode::WD155,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WD156: ItemRow = ItemRow {
    code: ItemCode::WD156,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV101: ItemRow = ItemRow {
    code: ItemCode::WV101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV102: ItemRow = ItemRow {
    code: ItemCode::WV102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV103: ItemRow = ItemRow {
    code: ItemCode::WV103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV104: ItemRow = ItemRow {
    code: ItemCode::WV104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV105: ItemRow = ItemRow {
    code: ItemCode::WV105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV106: ItemRow = ItemRow {
    code: ItemCode::WV106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV107: ItemRow = ItemRow {
    code: ItemCode::WV107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV108: ItemRow = ItemRow {
    code: ItemCode::WV108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV109: ItemRow = ItemRow {
    code: ItemCode::WV109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV110: ItemRow = ItemRow {
    code: ItemCode::WV110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV111: ItemRow = ItemRow {
    code: ItemCode::WV111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV112: ItemRow = ItemRow {
    code: ItemCode::WV112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV113: ItemRow = ItemRow {
    code: ItemCode::WV113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV114: ItemRow = ItemRow {
    code: ItemCode::WV114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV115: ItemRow = ItemRow {
    code: ItemCode::WV115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV116: ItemRow = ItemRow {
    code: ItemCode::WV116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV117: ItemRow = ItemRow {
    code: ItemCode::WV117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV118: ItemRow = ItemRow {
    code: ItemCode::WV118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV119: ItemRow = ItemRow {
    code: ItemCode::WV119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV120: ItemRow = ItemRow {
    code: ItemCode::WV120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV121: ItemRow = ItemRow {
    code: ItemCode::WV121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV122: ItemRow = ItemRow {
    code: ItemCode::WV122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV123: ItemRow = ItemRow {
    code: ItemCode::WV123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV124: ItemRow = ItemRow {
    code: ItemCode::WV124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV125: ItemRow = ItemRow {
    code: ItemCode::WV125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV126: ItemRow = ItemRow {
    code: ItemCode::WV126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV127: ItemRow = ItemRow {
    code: ItemCode::WV127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV128: ItemRow = ItemRow {
    code: ItemCode::WV128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV129: ItemRow = ItemRow {
    code: ItemCode::WV129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WV130: ItemRow = ItemRow {
    code: ItemCode::WV130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_OA203: ItemRow = ItemRow {
    code: ItemCode::OA203,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA204: ItemRow = ItemRow {
    code: ItemCode::OA204,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA205: ItemRow = ItemRow {
    code: ItemCode::OA205,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA206: ItemRow = ItemRow {
    code: ItemCode::OA206,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA207: ItemRow = ItemRow {
    code: ItemCode::OA207,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA208: ItemRow = ItemRow {
    code: ItemCode::OA208,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA209: ItemRow = ItemRow {
    code: ItemCode::OA209,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA210: ItemRow = ItemRow {
    code: ItemCode::OA210,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA211: ItemRow = ItemRow {
    code: ItemCode::OA211,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA212: ItemRow = ItemRow {
    code: ItemCode::OA212,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA213: ItemRow = ItemRow {
    code: ItemCode::OA213,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA214: ItemRow = ItemRow {
    code: ItemCode::OA214,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA215: ItemRow = ItemRow {
    code: ItemCode::OA215,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA216: ItemRow = ItemRow {
    code: ItemCode::OA216,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA217: ItemRow = ItemRow {
    code: ItemCode::OA217,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA218: ItemRow = ItemRow {
    code: ItemCode::OA218,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA219: ItemRow = ItemRow {
    code: ItemCode::OA219,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA220: ItemRow = ItemRow {
    code: ItemCode::OA220,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA221: ItemRow = ItemRow {
    code: ItemCode::OA221,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA222: ItemRow = ItemRow {
    code: ItemCode::OA222,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA223: ItemRow = ItemRow {
    code: ItemCode::OA223,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA224: ItemRow = ItemRow {
    code: ItemCode::OA224,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA225: ItemRow = ItemRow {
    code: ItemCode::OA225,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA226: ItemRow = ItemRow {
    code: ItemCode::OA226,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA227: ItemRow = ItemRow {
    code: ItemCode::OA227,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA228: ItemRow = ItemRow {
    code: ItemCode::OA228,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA229: ItemRow = ItemRow {
    code: ItemCode::OA229,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA231: ItemRow = ItemRow {
    code: ItemCode::OA231,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA232: ItemRow = ItemRow {
    code: ItemCode::OA232,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA233: ItemRow = ItemRow {
    code: ItemCode::OA233,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA234: ItemRow = ItemRow {
    code: ItemCode::OA234,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA251: ItemRow = ItemRow {
    code: ItemCode::OA251,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA252: ItemRow = ItemRow {
    code: ItemCode::OA252,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OM101: ItemRow = ItemRow {
    code: ItemCode::OM101,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM102: ItemRow = ItemRow {
    code: ItemCode::OM102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM103: ItemRow = ItemRow {
    code: ItemCode::OM103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM104: ItemRow = ItemRow {
    code: ItemCode::OM104,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM105: ItemRow = ItemRow {
    code: ItemCode::OM105,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM106: ItemRow = ItemRow {
    code: ItemCode::OM106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM107: ItemRow = ItemRow {
    code: ItemCode::OM107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM108: ItemRow = ItemRow {
    code: ItemCode::OM108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM109: ItemRow = ItemRow {
    code: ItemCode::OM109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM110: ItemRow = ItemRow {
    code: ItemCode::OM110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM111: ItemRow = ItemRow {
    code: ItemCode::OM111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM112: ItemRow = ItemRow {
    code: ItemCode::OM112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM113: ItemRow = ItemRow {
    code: ItemCode::OM113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM114: ItemRow = ItemRow {
    code: ItemCode::OM114,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM115: ItemRow = ItemRow {
    code: ItemCode::OM115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM116: ItemRow = ItemRow {
    code: ItemCode::OM116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM117: ItemRow = ItemRow {
    code: ItemCode::OM117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM118: ItemRow = ItemRow {
    code: ItemCode::OM118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM119: ItemRow = ItemRow {
    code: ItemCode::OM119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM120: ItemRow = ItemRow {
    code: ItemCode::OM120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM121: ItemRow = ItemRow {
    code: ItemCode::OM121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM122: ItemRow = ItemRow {
    code: ItemCode::OM122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM123: ItemRow = ItemRow {
    code: ItemCode::OM123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM124: ItemRow = ItemRow {
    code: ItemCode::OM124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM125: ItemRow = ItemRow {
    code: ItemCode::OM125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM126: ItemRow = ItemRow {
    code: ItemCode::OM126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM127: ItemRow = ItemRow {
    code: ItemCode::OM127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM128: ItemRow = ItemRow {
    code: ItemCode::OM128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM129: ItemRow = ItemRow {
    code: ItemCode::OM129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM130: ItemRow = ItemRow {
    code: ItemCode::OM130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OM131: ItemRow = ItemRow {
    code: ItemCode::OM131,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::MagicalStuff,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OR101: ItemRow = ItemRow {
    code: ItemCode::OR101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR102: ItemRow = ItemRow {
    code: ItemCode::OR102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR103: ItemRow = ItemRow {
    code: ItemCode::OR103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR104: ItemRow = ItemRow {
    code: ItemCode::OR104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR105: ItemRow = ItemRow {
    code: ItemCode::OR105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR106: ItemRow = ItemRow {
    code: ItemCode::OR106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR107: ItemRow = ItemRow {
    code: ItemCode::OR107,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR108: ItemRow = ItemRow {
    code: ItemCode::OR108,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR109: ItemRow = ItemRow {
    code: ItemCode::OR109,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR110: ItemRow = ItemRow {
    code: ItemCode::OR110,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR111: ItemRow = ItemRow {
    code: ItemCode::OR111,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR112: ItemRow = ItemRow {
    code: ItemCode::OR112,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR113: ItemRow = ItemRow {
    code: ItemCode::OR113,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR114: ItemRow = ItemRow {
    code: ItemCode::OR114,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR115: ItemRow = ItemRow {
    code: ItemCode::OR115,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR116: ItemRow = ItemRow {
    code: ItemCode::OR116,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR117: ItemRow = ItemRow {
    code: ItemCode::OR117,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR118: ItemRow = ItemRow {
    code: ItemCode::OR118,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR119: ItemRow = ItemRow {
    code: ItemCode::OR119,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR120: ItemRow = ItemRow {
    code: ItemCode::OR120,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA127: ItemRow = ItemRow {
    code: ItemCode::OA127,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA144: ItemRow = ItemRow {
    code: ItemCode::OA144,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR121: ItemRow = ItemRow {
    code: ItemCode::OR121,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR122: ItemRow = ItemRow {
    code: ItemCode::OR122,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR123: ItemRow = ItemRow {
    code: ItemCode::OR123,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR124: ItemRow = ItemRow {
    code: ItemCode::OR124,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR125: ItemRow = ItemRow {
    code: ItemCode::OR125,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR129: ItemRow = ItemRow {
    code: ItemCode::OR129,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS101: ItemRow = ItemRow {
    code: ItemCode::OS101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS102: ItemRow = ItemRow {
    code: ItemCode::OS102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS103: ItemRow = ItemRow {
    code: ItemCode::OS103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS104: ItemRow = ItemRow {
    code: ItemCode::OS104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS105: ItemRow = ItemRow {
    code: ItemCode::OS105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS106: ItemRow = ItemRow {
    code: ItemCode::OS106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS107: ItemRow = ItemRow {
    code: ItemCode::OS107,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS108: ItemRow = ItemRow {
    code: ItemCode::OS108,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS109: ItemRow = ItemRow {
    code: ItemCode::OS109,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS110: ItemRow = ItemRow {
    code: ItemCode::OS110,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS111: ItemRow = ItemRow {
    code: ItemCode::OS111,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS112: ItemRow = ItemRow {
    code: ItemCode::OS112,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS113: ItemRow = ItemRow {
    code: ItemCode::OS113,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS114: ItemRow = ItemRow {
    code: ItemCode::OS114,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS115: ItemRow = ItemRow {
    code: ItemCode::OS115,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS116: ItemRow = ItemRow {
    code: ItemCode::OS116,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS117: ItemRow = ItemRow {
    code: ItemCode::OS117,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS121: ItemRow = ItemRow {
    code: ItemCode::OS121,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS122: ItemRow = ItemRow {
    code: ItemCode::OS122,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS123: ItemRow = ItemRow {
    code: ItemCode::OS123,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS124: ItemRow = ItemRow {
    code: ItemCode::OS124,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS125: ItemRow = ItemRow {
    code: ItemCode::OS125,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS126: ItemRow = ItemRow {
    code: ItemCode::OS126,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS127: ItemRow = ItemRow {
    code: ItemCode::OS127,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS128: ItemRow = ItemRow {
    code: ItemCode::OS128,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS129: ItemRow = ItemRow {
    code: ItemCode::OS129,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS130: ItemRow = ItemRow {
    code: ItemCode::OS130,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS131: ItemRow = ItemRow {
    code: ItemCode::OS131,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS132: ItemRow = ItemRow {
    code: ItemCode::OS132,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS133: ItemRow = ItemRow {
    code: ItemCode::OS133,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS134: ItemRow = ItemRow {
    code: ItemCode::OS134,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO101: ItemRow = ItemRow {
    code: ItemCode::FO101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO102: ItemRow = ItemRow {
    code: ItemCode::FO102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO103: ItemRow = ItemRow {
    code: ItemCode::FO103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO104: ItemRow = ItemRow {
    code: ItemCode::FO104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO105: ItemRow = ItemRow {
    code: ItemCode::FO105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO106: ItemRow = ItemRow {
    code: ItemCode::FO106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO107: ItemRow = ItemRow {
    code: ItemCode::FO107,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO108: ItemRow = ItemRow {
    code: ItemCode::FO108,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO109: ItemRow = ItemRow {
    code: ItemCode::FO109,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO110: ItemRow = ItemRow {
    code: ItemCode::FO110,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO111: ItemRow = ItemRow {
    code: ItemCode::FO111,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO112: ItemRow = ItemRow {
    code: ItemCode::FO112,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO113: ItemRow = ItemRow {
    code: ItemCode::FO113,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO114: ItemRow = ItemRow {
    code: ItemCode::FO114,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO121: ItemRow = ItemRow {
    code: ItemCode::FO121,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO122: ItemRow = ItemRow {
    code: ItemCode::FO122,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO123: ItemRow = ItemRow {
    code: ItemCode::FO123,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO124: ItemRow = ItemRow {
    code: ItemCode::FO124,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO125: ItemRow = ItemRow {
    code: ItemCode::FO125,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO126: ItemRow = ItemRow {
    code: ItemCode::FO126,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO127: ItemRow = ItemRow {
    code: ItemCode::FO127,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO128: ItemRow = ItemRow {
    code: ItemCode::FO128,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO129: ItemRow = ItemRow {
    code: ItemCode::FO129,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO130: ItemRow = ItemRow {
    code: ItemCode::FO130,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO131: ItemRow = ItemRow {
    code: ItemCode::FO131,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO132: ItemRow = ItemRow {
    code: ItemCode::FO132,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO133: ItemRow = ItemRow {
    code: ItemCode::FO133,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO134: ItemRow = ItemRow {
    code: ItemCode::FO134,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO135: ItemRow = ItemRow {
    code: ItemCode::FO135,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO136: ItemRow = ItemRow {
    code: ItemCode::FO136,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_FO137: ItemRow = ItemRow {
    code: ItemCode::FO137,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::ForceOrb,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DA101: ItemRow = ItemRow {
    code: ItemCode::DA101,
    w: ITEMSIZE * 0,
    h: ITEMSIZE * 0,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA102: ItemRow = ItemRow {
    code: ItemCode::DA102,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA103: ItemRow = ItemRow {
    code: ItemCode::DA103,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA104: ItemRow = ItemRow {
    code: ItemCode::DA104,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA105: ItemRow = ItemRow {
    code: ItemCode::DA105,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA106: ItemRow = ItemRow {
    code: ItemCode::DA106,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA107: ItemRow = ItemRow {
    code: ItemCode::DA107,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA108: ItemRow = ItemRow {
    code: ItemCode::DA108,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA109: ItemRow = ItemRow {
    code: ItemCode::DA109,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA110: ItemRow = ItemRow {
    code: ItemCode::DA110,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA111: ItemRow = ItemRow {
    code: ItemCode::DA111,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA112: ItemRow = ItemRow {
    code: ItemCode::DA112,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA113: ItemRow = ItemRow {
    code: ItemCode::DA113,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA114: ItemRow = ItemRow {
    code: ItemCode::DA114,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA115: ItemRow = ItemRow {
    code: ItemCode::DA115,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA116: ItemRow = ItemRow {
    code: ItemCode::DA116,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA117: ItemRow = ItemRow {
    code: ItemCode::DA117,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA118: ItemRow = ItemRow {
    code: ItemCode::DA118,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA119: ItemRow = ItemRow {
    code: ItemCode::DA119,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA120: ItemRow = ItemRow {
    code: ItemCode::DA120,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA121: ItemRow = ItemRow {
    code: ItemCode::DA121,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA122: ItemRow = ItemRow {
    code: ItemCode::DA122,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA123: ItemRow = ItemRow {
    code: ItemCode::DA123,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA124: ItemRow = ItemRow {
    code: ItemCode::DA124,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA125: ItemRow = ItemRow {
    code: ItemCode::DA125,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA126: ItemRow = ItemRow {
    code: ItemCode::DA126,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA127: ItemRow = ItemRow {
    code: ItemCode::DA127,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA128: ItemRow = ItemRow {
    code: ItemCode::DA128,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA129: ItemRow = ItemRow {
    code: ItemCode::DA129,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA301: ItemRow = ItemRow {
    code: ItemCode::DA301,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA302: ItemRow = ItemRow {
    code: ItemCode::DA302,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA303: ItemRow = ItemRow {
    code: ItemCode::DA303,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA304: ItemRow = ItemRow {
    code: ItemCode::DA304,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA305: ItemRow = ItemRow {
    code: ItemCode::DA305,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA306: ItemRow = ItemRow {
    code: ItemCode::DA306,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA307: ItemRow = ItemRow {
    code: ItemCode::DA307,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA308: ItemRow = ItemRow {
    code: ItemCode::DA308,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA201: ItemRow = ItemRow {
    code: ItemCode::DA201,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA202: ItemRow = ItemRow {
    code: ItemCode::DA202,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA203: ItemRow = ItemRow {
    code: ItemCode::DA203,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA204: ItemRow = ItemRow {
    code: ItemCode::DA204,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA205: ItemRow = ItemRow {
    code: ItemCode::DA205,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA206: ItemRow = ItemRow {
    code: ItemCode::DA206,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA207: ItemRow = ItemRow {
    code: ItemCode::DA207,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA208: ItemRow = ItemRow {
    code: ItemCode::DA208,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA209: ItemRow = ItemRow {
    code: ItemCode::DA209,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA210: ItemRow = ItemRow {
    code: ItemCode::DA210,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA211: ItemRow = ItemRow {
    code: ItemCode::DA211,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA212: ItemRow = ItemRow {
    code: ItemCode::DA212,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA213: ItemRow = ItemRow {
    code: ItemCode::DA213,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA214: ItemRow = ItemRow {
    code: ItemCode::DA214,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA215: ItemRow = ItemRow {
    code: ItemCode::DA215,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA216: ItemRow = ItemRow {
    code: ItemCode::DA216,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA217: ItemRow = ItemRow {
    code: ItemCode::DA217,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA218: ItemRow = ItemRow {
    code: ItemCode::DA218,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA219: ItemRow = ItemRow {
    code: ItemCode::DA219,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA220: ItemRow = ItemRow {
    code: ItemCode::DA220,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA221: ItemRow = ItemRow {
    code: ItemCode::DA221,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA222: ItemRow = ItemRow {
    code: ItemCode::DA222,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA223: ItemRow = ItemRow {
    code: ItemCode::DA223,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA224: ItemRow = ItemRow {
    code: ItemCode::DA224,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA225: ItemRow = ItemRow {
    code: ItemCode::DA225,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA226: ItemRow = ItemRow {
    code: ItemCode::DA226,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA227: ItemRow = ItemRow {
    code: ItemCode::DA227,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA228: ItemRow = ItemRow {
    code: ItemCode::DA228,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA229: ItemRow = ItemRow {
    code: ItemCode::DA229,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA401: ItemRow = ItemRow {
    code: ItemCode::DA401,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA402: ItemRow = ItemRow {
    code: ItemCode::DA402,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA403: ItemRow = ItemRow {
    code: ItemCode::DA403,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA404: ItemRow = ItemRow {
    code: ItemCode::DA404,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA405: ItemRow = ItemRow {
    code: ItemCode::DA405,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA406: ItemRow = ItemRow {
    code: ItemCode::DA406,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA407: ItemRow = ItemRow {
    code: ItemCode::DA407,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA408: ItemRow = ItemRow {
    code: ItemCode::DA408,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA131: ItemRow = ItemRow {
    code: ItemCode::DA131,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA132: ItemRow = ItemRow {
    code: ItemCode::DA132,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA133: ItemRow = ItemRow {
    code: ItemCode::DA133,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA134: ItemRow = ItemRow {
    code: ItemCode::DA134,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA135: ItemRow = ItemRow {
    code: ItemCode::DA135,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA136: ItemRow = ItemRow {
    code: ItemCode::DA136,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA137: ItemRow = ItemRow {
    code: ItemCode::DA137,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA138: ItemRow = ItemRow {
    code: ItemCode::DA138,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA139: ItemRow = ItemRow {
    code: ItemCode::DA139,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA140: ItemRow = ItemRow {
    code: ItemCode::DA140,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA141: ItemRow = ItemRow {
    code: ItemCode::DA141,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA142: ItemRow = ItemRow {
    code: ItemCode::DA142,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA143: ItemRow = ItemRow {
    code: ItemCode::DA143,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA144: ItemRow = ItemRow {
    code: ItemCode::DA144,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA145: ItemRow = ItemRow {
    code: ItemCode::DA145,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA146: ItemRow = ItemRow {
    code: ItemCode::DA146,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA231: ItemRow = ItemRow {
    code: ItemCode::DA231,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA232: ItemRow = ItemRow {
    code: ItemCode::DA232,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA233: ItemRow = ItemRow {
    code: ItemCode::DA233,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA234: ItemRow = ItemRow {
    code: ItemCode::DA234,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA235: ItemRow = ItemRow {
    code: ItemCode::DA235,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA236: ItemRow = ItemRow {
    code: ItemCode::DA236,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA237: ItemRow = ItemRow {
    code: ItemCode::DA237,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA238: ItemRow = ItemRow {
    code: ItemCode::DA238,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA239: ItemRow = ItemRow {
    code: ItemCode::DA239,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA240: ItemRow = ItemRow {
    code: ItemCode::DA240,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA241: ItemRow = ItemRow {
    code: ItemCode::DA241,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA242: ItemRow = ItemRow {
    code: ItemCode::DA242,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA243: ItemRow = ItemRow {
    code: ItemCode::DA243,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA244: ItemRow = ItemRow {
    code: ItemCode::DA244,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA245: ItemRow = ItemRow {
    code: ItemCode::DA245,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA246: ItemRow = ItemRow {
    code: ItemCode::DA246,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA147: ItemRow = ItemRow {
    code: ItemCode::DA147,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA247: ItemRow = ItemRow {
    code: ItemCode::DA247,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA148: ItemRow = ItemRow {
    code: ItemCode::DA148,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA248: ItemRow = ItemRow {
    code: ItemCode::DA248,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA149: ItemRow = ItemRow {
    code: ItemCode::DA149,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA249: ItemRow = ItemRow {
    code: ItemCode::DA249,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA150: ItemRow = ItemRow {
    code: ItemCode::DA150,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA250: ItemRow = ItemRow {
    code: ItemCode::DA250,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA151: ItemRow = ItemRow {
    code: ItemCode::DA151,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA251: ItemRow = ItemRow {
    code: ItemCode::DA251,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA152: ItemRow = ItemRow {
    code: ItemCode::DA152,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA252: ItemRow = ItemRow {
    code: ItemCode::DA252,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA153: ItemRow = ItemRow {
    code: ItemCode::DA153,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA253: ItemRow = ItemRow {
    code: ItemCode::DA253,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA254: ItemRow = ItemRow {
    code: ItemCode::DA254,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA255: ItemRow = ItemRow {
    code: ItemCode::DA255,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA162: ItemRow = ItemRow {
    code: ItemCode::DA162,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA163: ItemRow = ItemRow {
    code: ItemCode::DA163,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA262: ItemRow = ItemRow {
    code: ItemCode::DA262,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA263: ItemRow = ItemRow {
    code: ItemCode::DA263,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA176: ItemRow = ItemRow {
    code: ItemCode::DA176,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA177: ItemRow = ItemRow {
    code: ItemCode::DA177,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA276: ItemRow = ItemRow {
    code: ItemCode::DA276,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA277: ItemRow = ItemRow {
    code: ItemCode::DA277,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DB101: ItemRow = ItemRow {
    code: ItemCode::DB101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB102: ItemRow = ItemRow {
    code: ItemCode::DB102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB103: ItemRow = ItemRow {
    code: ItemCode::DB103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB104: ItemRow = ItemRow {
    code: ItemCode::DB104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB105: ItemRow = ItemRow {
    code: ItemCode::DB105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB106: ItemRow = ItemRow {
    code: ItemCode::DB106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB107: ItemRow = ItemRow {
    code: ItemCode::DB107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB108: ItemRow = ItemRow {
    code: ItemCode::DB108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB109: ItemRow = ItemRow {
    code: ItemCode::DB109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB110: ItemRow = ItemRow {
    code: ItemCode::DB110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB111: ItemRow = ItemRow {
    code: ItemCode::DB111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB112: ItemRow = ItemRow {
    code: ItemCode::DB112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB113: ItemRow = ItemRow {
    code: ItemCode::DB113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB114: ItemRow = ItemRow {
    code: ItemCode::DB114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB115: ItemRow = ItemRow {
    code: ItemCode::DB115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB116: ItemRow = ItemRow {
    code: ItemCode::DB116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB117: ItemRow = ItemRow {
    code: ItemCode::DB117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB118: ItemRow = ItemRow {
    code: ItemCode::DB118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB119: ItemRow = ItemRow {
    code: ItemCode::DB119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB120: ItemRow = ItemRow {
    code: ItemCode::DB120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB121: ItemRow = ItemRow {
    code: ItemCode::DB121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB122: ItemRow = ItemRow {
    code: ItemCode::DB122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB123: ItemRow = ItemRow {
    code: ItemCode::DB123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB124: ItemRow = ItemRow {
    code: ItemCode::DB124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB125: ItemRow = ItemRow {
    code: ItemCode::DB125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB126: ItemRow = ItemRow {
    code: ItemCode::DB126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB127: ItemRow = ItemRow {
    code: ItemCode::DB127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB128: ItemRow = ItemRow {
    code: ItemCode::DB128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB129: ItemRow = ItemRow {
    code: ItemCode::DB129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB130: ItemRow = ItemRow {
    code: ItemCode::DB130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB131: ItemRow = ItemRow {
    code: ItemCode::DB131,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB132: ItemRow = ItemRow {
    code: ItemCode::DB132,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB133: ItemRow = ItemRow {
    code: ItemCode::DB133,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB134: ItemRow = ItemRow {
    code: ItemCode::DB134,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB150: ItemRow = ItemRow {
    code: ItemCode::DB150,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG101: ItemRow = ItemRow {
    code: ItemCode::DG101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG102: ItemRow = ItemRow {
    code: ItemCode::DG102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG103: ItemRow = ItemRow {
    code: ItemCode::DG103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG104: ItemRow = ItemRow {
    code: ItemCode::DG104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG105: ItemRow = ItemRow {
    code: ItemCode::DG105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG106: ItemRow = ItemRow {
    code: ItemCode::DG106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG107: ItemRow = ItemRow {
    code: ItemCode::DG107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG108: ItemRow = ItemRow {
    code: ItemCode::DG108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG109: ItemRow = ItemRow {
    code: ItemCode::DG109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG110: ItemRow = ItemRow {
    code: ItemCode::DG110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG111: ItemRow = ItemRow {
    code: ItemCode::DG111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG112: ItemRow = ItemRow {
    code: ItemCode::DG112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG113: ItemRow = ItemRow {
    code: ItemCode::DG113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG114: ItemRow = ItemRow {
    code: ItemCode::DG114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG115: ItemRow = ItemRow {
    code: ItemCode::DG115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG116: ItemRow = ItemRow {
    code: ItemCode::DG116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG117: ItemRow = ItemRow {
    code: ItemCode::DG117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG118: ItemRow = ItemRow {
    code: ItemCode::DG118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG119: ItemRow = ItemRow {
    code: ItemCode::DG119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG120: ItemRow = ItemRow {
    code: ItemCode::DG120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG121: ItemRow = ItemRow {
    code: ItemCode::DG121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG122: ItemRow = ItemRow {
    code: ItemCode::DG122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG123: ItemRow = ItemRow {
    code: ItemCode::DG123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG124: ItemRow = ItemRow {
    code: ItemCode::DG124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG125: ItemRow = ItemRow {
    code: ItemCode::DG125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG126: ItemRow = ItemRow {
    code: ItemCode::DG126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG127: ItemRow = ItemRow {
    code: ItemCode::DG127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG128: ItemRow = ItemRow {
    code: ItemCode::DG128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG129: ItemRow = ItemRow {
    code: ItemCode::DG129,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG130: ItemRow = ItemRow {
    code: ItemCode::DG130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DS101: ItemRow = ItemRow {
    code: ItemCode::DS101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS102: ItemRow = ItemRow {
    code: ItemCode::DS102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS103: ItemRow = ItemRow {
    code: ItemCode::DS103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS104: ItemRow = ItemRow {
    code: ItemCode::DS104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS105: ItemRow = ItemRow {
    code: ItemCode::DS105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS106: ItemRow = ItemRow {
    code: ItemCode::DS106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 3,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS107: ItemRow = ItemRow {
    code: ItemCode::DS107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS108: ItemRow = ItemRow {
    code: ItemCode::DS108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS109: ItemRow = ItemRow {
    code: ItemCode::DS109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS110: ItemRow = ItemRow {
    code: ItemCode::DS110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS111: ItemRow = ItemRow {
    code: ItemCode::DS111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS112: ItemRow = ItemRow {
    code: ItemCode::DS112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS113: ItemRow = ItemRow {
    code: ItemCode::DS113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS114: ItemRow = ItemRow {
    code: ItemCode::DS114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS115: ItemRow = ItemRow {
    code: ItemCode::DS115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS116: ItemRow = ItemRow {
    code: ItemCode::DS116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS117: ItemRow = ItemRow {
    code: ItemCode::DS117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS118: ItemRow = ItemRow {
    code: ItemCode::DS118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS119: ItemRow = ItemRow {
    code: ItemCode::DS119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS120: ItemRow = ItemRow {
    code: ItemCode::DS120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS121: ItemRow = ItemRow {
    code: ItemCode::DS121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS122: ItemRow = ItemRow {
    code: ItemCode::DS122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS123: ItemRow = ItemRow {
    code: ItemCode::DS123,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS124: ItemRow = ItemRow {
    code: ItemCode::DS124,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS125: ItemRow = ItemRow {
    code: ItemCode::DS125,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS126: ItemRow = ItemRow {
    code: ItemCode::DS126,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS127: ItemRow = ItemRow {
    code: ItemCode::DS127,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS128: ItemRow = ItemRow {
    code: ItemCode::DS128,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_DS130: ItemRow = ItemRow {
    code: ItemCode::DS130,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OR201: ItemRow = ItemRow {
    code: ItemCode::OR201,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR202: ItemRow = ItemRow {
    code: ItemCode::OR202,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR203: ItemRow = ItemRow {
    code: ItemCode::OR203,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR204: ItemRow = ItemRow {
    code: ItemCode::OR204,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR205: ItemRow = ItemRow {
    code: ItemCode::OR205,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR206: ItemRow = ItemRow {
    code: ItemCode::OR206,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR207: ItemRow = ItemRow {
    code: ItemCode::OR207,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR208: ItemRow = ItemRow {
    code: ItemCode::OR208,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR209: ItemRow = ItemRow {
    code: ItemCode::OR209,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR210: ItemRow = ItemRow {
    code: ItemCode::OR210,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR211: ItemRow = ItemRow {
    code: ItemCode::OR211,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR212: ItemRow = ItemRow {
    code: ItemCode::OR212,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR213: ItemRow = ItemRow {
    code: ItemCode::OR213,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR214: ItemRow = ItemRow {
    code: ItemCode::OR214,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR215: ItemRow = ItemRow {
    code: ItemCode::OR215,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR216: ItemRow = ItemRow {
    code: ItemCode::OR216,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR217: ItemRow = ItemRow {
    code: ItemCode::OR217,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR218: ItemRow = ItemRow {
    code: ItemCode::OR218,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR219: ItemRow = ItemRow {
    code: ItemCode::OR219,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR220: ItemRow = ItemRow {
    code: ItemCode::OR220,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR221: ItemRow = ItemRow {
    code: ItemCode::OR221,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR222: ItemRow = ItemRow {
    code: ItemCode::OR222,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR223: ItemRow = ItemRow {
    code: ItemCode::OR223,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR224: ItemRow = ItemRow {
    code: ItemCode::OR224,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR225: ItemRow = ItemRow {
    code: ItemCode::OR225,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR227: ItemRow = ItemRow {
    code: ItemCode::OR227,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR228: ItemRow = ItemRow {
    code: ItemCode::OR228,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR229: ItemRow = ItemRow {
    code: ItemCode::OR229,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR230: ItemRow = ItemRow {
    code: ItemCode::OR230,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR231: ItemRow = ItemRow {
    code: ItemCode::OR231,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR232: ItemRow = ItemRow {
    code: ItemCode::OR232,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR233: ItemRow = ItemRow {
    code: ItemCode::OR233,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR234: ItemRow = ItemRow {
    code: ItemCode::OR234,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR235: ItemRow = ItemRow {
    code: ItemCode::OR235,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR236: ItemRow = ItemRow {
    code: ItemCode::OR236,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR237: ItemRow = ItemRow {
    code: ItemCode::OR237,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR238: ItemRow = ItemRow {
    code: ItemCode::OR238,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR239: ItemRow = ItemRow {
    code: ItemCode::OR239,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR240: ItemRow = ItemRow {
    code: ItemCode::OR240,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR248: ItemRow = ItemRow {
    code: ItemCode::OR248,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OR251: ItemRow = ItemRow {
    code: ItemCode::OR251,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PM101: ItemRow = ItemRow {
    code: ItemCode::PM101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PM102: ItemRow = ItemRow {
    code: ItemCode::PM102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PM103: ItemRow = ItemRow {
    code: ItemCode::PM103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PM104: ItemRow = ItemRow {
    code: ItemCode::PM104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PM105: ItemRow = ItemRow {
    code: ItemCode::PM105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PL101: ItemRow = ItemRow {
    code: ItemCode::PL101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PL102: ItemRow = ItemRow {
    code: ItemCode::PL102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PL103: ItemRow = ItemRow {
    code: ItemCode::PL103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PL104: ItemRow = ItemRow {
    code: ItemCode::PL104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PL105: ItemRow = ItemRow {
    code: ItemCode::PL105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PS101: ItemRow = ItemRow {
    code: ItemCode::PS101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PS102: ItemRow = ItemRow {
    code: ItemCode::PS102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PS103: ItemRow = ItemRow {
    code: ItemCode::PS103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PS104: ItemRow = ItemRow {
    code: ItemCode::PS104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PS105: ItemRow = ItemRow {
    code: ItemCode::PS105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Potion,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EC101: ItemRow = ItemRow {
    code: ItemCode::EC101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EC102: ItemRow = ItemRow {
    code: ItemCode::EC102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EC103: ItemRow = ItemRow {
    code: ItemCode::EC103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EC104: ItemRow = ItemRow {
    code: ItemCode::EC104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EC105: ItemRow = ItemRow {
    code: ItemCode::EC105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EC106: ItemRow = ItemRow {
    code: ItemCode::EC106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT101: ItemRow = ItemRow {
    code: ItemCode::QT101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT102: ItemRow = ItemRow {
    code: ItemCode::QT102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT103: ItemRow = ItemRow {
    code: ItemCode::QT103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT104: ItemRow = ItemRow {
    code: ItemCode::QT104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT105: ItemRow = ItemRow {
    code: ItemCode::QT105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT106: ItemRow = ItemRow {
    code: ItemCode::QT106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT107: ItemRow = ItemRow {
    code: ItemCode::QT107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT108: ItemRow = ItemRow {
    code: ItemCode::QT108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT109: ItemRow = ItemRow {
    code: ItemCode::QT109,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT110: ItemRow = ItemRow {
    code: ItemCode::QT110,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT111: ItemRow = ItemRow {
    code: ItemCode::QT111,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT112: ItemRow = ItemRow {
    code: ItemCode::QT112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT113: ItemRow = ItemRow {
    code: ItemCode::QT113,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT114: ItemRow = ItemRow {
    code: ItemCode::QT114,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT115: ItemRow = ItemRow {
    code: ItemCode::QT115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QT116: ItemRow = ItemRow {
    code: ItemCode::QT116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP101: ItemRow = ItemRow {
    code: ItemCode::SP101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP102: ItemRow = ItemRow {
    code: ItemCode::SP102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP103: ItemRow = ItemRow {
    code: ItemCode::SP103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP105: ItemRow = ItemRow {
    code: ItemCode::SP105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP106: ItemRow = ItemRow {
    code: ItemCode::SP106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP107: ItemRow = ItemRow {
    code: ItemCode::SP107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP108: ItemRow = ItemRow {
    code: ItemCode::SP108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP109: ItemRow = ItemRow {
    code: ItemCode::SP109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP110: ItemRow = ItemRow {
    code: ItemCode::SP110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP115: ItemRow = ItemRow {
    code: ItemCode::SP115,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP126: ItemRow = ItemRow {
    code: ItemCode::SP126,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP127: ItemRow = ItemRow {
    code: ItemCode::SP127,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP128: ItemRow = ItemRow {
    code: ItemCode::SP128,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP129: ItemRow = ItemRow {
    code: ItemCode::SP129,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP130: ItemRow = ItemRow {
    code: ItemCode::SP130,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP131: ItemRow = ItemRow {
    code: ItemCode::SP131,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP132: ItemRow = ItemRow {
    code: ItemCode::SP132,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP133: ItemRow = ItemRow {
    code: ItemCode::SP133,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP134: ItemRow = ItemRow {
    code: ItemCode::SP134,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP135: ItemRow = ItemRow {
    code: ItemCode::SP135,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP136: ItemRow = ItemRow {
    code: ItemCode::SP136,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP137: ItemRow = ItemRow {
    code: ItemCode::SP137,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP138: ItemRow = ItemRow {
    code: ItemCode::SP138,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP139: ItemRow = ItemRow {
    code: ItemCode::SP139,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP140: ItemRow = ItemRow {
    code: ItemCode::SP140,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP142: ItemRow = ItemRow {
    code: ItemCode::SP142,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP143: ItemRow = ItemRow {
    code: ItemCode::SP143,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP144: ItemRow = ItemRow {
    code: ItemCode::SP144,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP145: ItemRow = ItemRow {
    code: ItemCode::SP145,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP146: ItemRow = ItemRow {
    code: ItemCode::SP146,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP147: ItemRow = ItemRow {
    code: ItemCode::SP147,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP148: ItemRow = ItemRow {
    code: ItemCode::SP148,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP149: ItemRow = ItemRow {
    code: ItemCode::SP149,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP150: ItemRow = ItemRow {
    code: ItemCode::SP150,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP151: ItemRow = ItemRow {
    code: ItemCode::SP151,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP152: ItemRow = ItemRow {
    code: ItemCode::SP152,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP153: ItemRow = ItemRow {
    code: ItemCode::SP153,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP154: ItemRow = ItemRow {
    code: ItemCode::SP154,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP155: ItemRow = ItemRow {
    code: ItemCode::SP155,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP156: ItemRow = ItemRow {
    code: ItemCode::SP156,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP157: ItemRow = ItemRow {
    code: ItemCode::SP157,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP160: ItemRow = ItemRow {
    code: ItemCode::SP160,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP161: ItemRow = ItemRow {
    code: ItemCode::SP161,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP162: ItemRow = ItemRow {
    code: ItemCode::SP162,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SP163: ItemRow = ItemRow {
    code: ItemCode::SP163,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP101: ItemRow = ItemRow {
    code: ItemCode::GP101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP102: ItemRow = ItemRow {
    code: ItemCode::GP102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP103: ItemRow = ItemRow {
    code: ItemCode::GP103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP104: ItemRow = ItemRow {
    code: ItemCode::GP104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP105: ItemRow = ItemRow {
    code: ItemCode::GP105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP106: ItemRow = ItemRow {
    code: ItemCode::GP106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP107: ItemRow = ItemRow {
    code: ItemCode::GP107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP108: ItemRow = ItemRow {
    code: ItemCode::GP108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP109: ItemRow = ItemRow {
    code: ItemCode::GP109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP110: ItemRow = ItemRow {
    code: ItemCode::GP110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP111: ItemRow = ItemRow {
    code: ItemCode::GP111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP112: ItemRow = ItemRow {
    code: ItemCode::GP112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP113: ItemRow = ItemRow {
    code: ItemCode::GP113,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP114: ItemRow = ItemRow {
    code: ItemCode::GP114,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP115: ItemRow = ItemRow {
    code: ItemCode::GP115,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP116: ItemRow = ItemRow {
    code: ItemCode::GP116,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP117: ItemRow = ItemRow {
    code: ItemCode::GP117,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP118: ItemRow = ItemRow {
    code: ItemCode::GP118,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP119: ItemRow = ItemRow {
    code: ItemCode::GP119,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP120: ItemRow = ItemRow {
    code: ItemCode::GP120,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP121: ItemRow = ItemRow {
    code: ItemCode::GP121,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP122: ItemRow = ItemRow {
    code: ItemCode::GP122,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP135: ItemRow = ItemRow {
    code: ItemCode::GP135,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP136: ItemRow = ItemRow {
    code: ItemCode::GP136,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP137: ItemRow = ItemRow {
    code: ItemCode::GP137,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP201: ItemRow = ItemRow {
    code: ItemCode::GP201,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP202: ItemRow = ItemRow {
    code: ItemCode::GP202,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP203: ItemRow = ItemRow {
    code: ItemCode::GP203,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP204: ItemRow = ItemRow {
    code: ItemCode::GP204,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP205: ItemRow = ItemRow {
    code: ItemCode::GP205,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP206: ItemRow = ItemRow {
    code: ItemCode::GP206,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP207: ItemRow = ItemRow {
    code: ItemCode::GP207,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP208: ItemRow = ItemRow {
    code: ItemCode::GP208,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP209: ItemRow = ItemRow {
    code: ItemCode::GP209,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP210: ItemRow = ItemRow {
    code: ItemCode::GP210,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GP211: ItemRow = ItemRow {
    code: ItemCode::GP211,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QW101: ItemRow = ItemRow {
    code: ItemCode::QW101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QW102: ItemRow = ItemRow {
    code: ItemCode::QW102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QW103: ItemRow = ItemRow {
    code: ItemCode::QW103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QW104: ItemRow = ItemRow {
    code: ItemCode::QW104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QW105: ItemRow = ItemRow {
    code: ItemCode::QW105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_QW106: ItemRow = ItemRow {
    code: ItemCode::QW106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_MA101: ItemRow = ItemRow {
    code: ItemCode::MA101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_MA201: ItemRow = ItemRow {
    code: ItemCode::MA201,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_MA202: ItemRow = ItemRow {
    code: ItemCode::MA202,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GF101: ItemRow = ItemRow {
    code: ItemCode::GF101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GF103: ItemRow = ItemRow {
    code: ItemCode::GF103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GF104: ItemRow = ItemRow {
    code: ItemCode::GF104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GF105: ItemRow = ItemRow {
    code: ItemCode::GF105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GF106: ItemRow = ItemRow {
    code: ItemCode::GF106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GF107: ItemRow = ItemRow {
    code: ItemCode::GF107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GF108: ItemRow = ItemRow {
    code: ItemCode::GF108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GF102: ItemRow = ItemRow {
    code: ItemCode::GF102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SD201: ItemRow = ItemRow {
    code: ItemCode::SD201,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SD202: ItemRow = ItemRow {
    code: ItemCode::SD202,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SD203: ItemRow = ItemRow {
    code: ItemCode::SD203,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SD204: ItemRow = ItemRow {
    code: ItemCode::SD204,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SD205: ItemRow = ItemRow {
    code: ItemCode::SD205,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SD206: ItemRow = ItemRow {
    code: ItemCode::SD206,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SD207: ItemRow = ItemRow {
    code: ItemCode::SD207,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BS101: ItemRow = ItemRow {
    code: ItemCode::BS101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BS102: ItemRow = ItemRow {
    code: ItemCode::BS102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BS103: ItemRow = ItemRow {
    code: ItemCode::BS103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC101: ItemRow = ItemRow {
    code: ItemCode::BC101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC102: ItemRow = ItemRow {
    code: ItemCode::BC102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC103: ItemRow = ItemRow {
    code: ItemCode::BC103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC104: ItemRow = ItemRow {
    code: ItemCode::BC104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC105: ItemRow = ItemRow {
    code: ItemCode::BC105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC106: ItemRow = ItemRow {
    code: ItemCode::BC106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC107: ItemRow = ItemRow {
    code: ItemCode::BC107,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC108: ItemRow = ItemRow {
    code: ItemCode::BC108,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC109: ItemRow = ItemRow {
    code: ItemCode::BC109,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC110: ItemRow = ItemRow {
    code: ItemCode::BC110,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC111: ItemRow = ItemRow {
    code: ItemCode::BC111,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC112: ItemRow = ItemRow {
    code: ItemCode::BC112,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC113: ItemRow = ItemRow {
    code: ItemCode::BC113,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC114: ItemRow = ItemRow {
    code: ItemCode::BC114,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC115: ItemRow = ItemRow {
    code: ItemCode::BC115,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC116: ItemRow = ItemRow {
    code: ItemCode::BC116,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC121: ItemRow = ItemRow {
    code: ItemCode::BC121,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC122: ItemRow = ItemRow {
    code: ItemCode::BC122,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC123: ItemRow = ItemRow {
    code: ItemCode::BC123,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC124: ItemRow = ItemRow {
    code: ItemCode::BC124,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC125: ItemRow = ItemRow {
    code: ItemCode::BC125,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC126: ItemRow = ItemRow {
    code: ItemCode::BC126,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC127: ItemRow = ItemRow {
    code: ItemCode::BC127,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC128: ItemRow = ItemRow {
    code: ItemCode::BC128,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC129: ItemRow = ItemRow {
    code: ItemCode::BC129,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC130: ItemRow = ItemRow {
    code: ItemCode::BC130,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC131: ItemRow = ItemRow {
    code: ItemCode::BC131,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BC132: ItemRow = ItemRow {
    code: ItemCode::BC132,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI101: ItemRow = ItemRow {
    code: ItemCode::BI101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI102: ItemRow = ItemRow {
    code: ItemCode::BI102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI103: ItemRow = ItemRow {
    code: ItemCode::BI103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI104: ItemRow = ItemRow {
    code: ItemCode::BI104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI105: ItemRow = ItemRow {
    code: ItemCode::BI105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI106: ItemRow = ItemRow {
    code: ItemCode::BI106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI107: ItemRow = ItemRow {
    code: ItemCode::BI107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI108: ItemRow = ItemRow {
    code: ItemCode::BI108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI109: ItemRow = ItemRow {
    code: ItemCode::BI109,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI110: ItemRow = ItemRow {
    code: ItemCode::BI110,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI111: ItemRow = ItemRow {
    code: ItemCode::BI111,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI112: ItemRow = ItemRow {
    code: ItemCode::BI112,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI113: ItemRow = ItemRow {
    code: ItemCode::BI113,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI114: ItemRow = ItemRow {
    code: ItemCode::BI114,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI115: ItemRow = ItemRow {
    code: ItemCode::BI115,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI116: ItemRow = ItemRow {
    code: ItemCode::BI116,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI117: ItemRow = ItemRow {
    code: ItemCode::BI117,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI118: ItemRow = ItemRow {
    code: ItemCode::BI118,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI119: ItemRow = ItemRow {
    code: ItemCode::BI119,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI120: ItemRow = ItemRow {
    code: ItemCode::BI120,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI121: ItemRow = ItemRow {
    code: ItemCode::BI121,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI122: ItemRow = ItemRow {
    code: ItemCode::BI122,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI123: ItemRow = ItemRow {
    code: ItemCode::BI123,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI124: ItemRow = ItemRow {
    code: ItemCode::BI124,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI125: ItemRow = ItemRow {
    code: ItemCode::BI125,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI126: ItemRow = ItemRow {
    code: ItemCode::BI126,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI127: ItemRow = ItemRow {
    code: ItemCode::BI127,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI128: ItemRow = ItemRow {
    code: ItemCode::BI128,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI129: ItemRow = ItemRow {
    code: ItemCode::BI129,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI130: ItemRow = ItemRow {
    code: ItemCode::BI130,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI131: ItemRow = ItemRow {
    code: ItemCode::BI131,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI132: ItemRow = ItemRow {
    code: ItemCode::BI132,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI133: ItemRow = ItemRow {
    code: ItemCode::BI133,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI134: ItemRow = ItemRow {
    code: ItemCode::BI134,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI136: ItemRow = ItemRow {
    code: ItemCode::BI136,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI137: ItemRow = ItemRow {
    code: ItemCode::BI137,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI138: ItemRow = ItemRow {
    code: ItemCode::BI138,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI139: ItemRow = ItemRow {
    code: ItemCode::BI139,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI140: ItemRow = ItemRow {
    code: ItemCode::BI140,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI141: ItemRow = ItemRow {
    code: ItemCode::BI141,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI142: ItemRow = ItemRow {
    code: ItemCode::BI142,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI143: ItemRow = ItemRow {
    code: ItemCode::BI143,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI144: ItemRow = ItemRow {
    code: ItemCode::BI144,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI145: ItemRow = ItemRow {
    code: ItemCode::BI145,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI146: ItemRow = ItemRow {
    code: ItemCode::BI146,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI147: ItemRow = ItemRow {
    code: ItemCode::BI147,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI148: ItemRow = ItemRow {
    code: ItemCode::BI148,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI149: ItemRow = ItemRow {
    code: ItemCode::BI149,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI150: ItemRow = ItemRow {
    code: ItemCode::BI150,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI151: ItemRow = ItemRow {
    code: ItemCode::BI151,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI152: ItemRow = ItemRow {
    code: ItemCode::BI152,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI153: ItemRow = ItemRow {
    code: ItemCode::BI153,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI154: ItemRow = ItemRow {
    code: ItemCode::BI154,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI155: ItemRow = ItemRow {
    code: ItemCode::BI155,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI160: ItemRow = ItemRow {
    code: ItemCode::BI160,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI161: ItemRow = ItemRow {
    code: ItemCode::BI161,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI162: ItemRow = ItemRow {
    code: ItemCode::BI162,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI163: ItemRow = ItemRow {
    code: ItemCode::BI163,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI164: ItemRow = ItemRow {
    code: ItemCode::BI164,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI165: ItemRow = ItemRow {
    code: ItemCode::BI165,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI166: ItemRow = ItemRow {
    code: ItemCode::BI166,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI167: ItemRow = ItemRow {
    code: ItemCode::BI167,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI168: ItemRow = ItemRow {
    code: ItemCode::BI168,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI169: ItemRow = ItemRow {
    code: ItemCode::BI169,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI170: ItemRow = ItemRow {
    code: ItemCode::BI170,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI171: ItemRow = ItemRow {
    code: ItemCode::BI171,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI172: ItemRow = ItemRow {
    code: ItemCode::BI172,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI173: ItemRow = ItemRow {
    code: ItemCode::BI173,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI174: ItemRow = ItemRow {
    code: ItemCode::BI174,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI175: ItemRow = ItemRow {
    code: ItemCode::BI175,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI176: ItemRow = ItemRow {
    code: ItemCode::BI176,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI177: ItemRow = ItemRow {
    code: ItemCode::BI177,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI178: ItemRow = ItemRow {
    code: ItemCode::BI178,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI179: ItemRow = ItemRow {
    code: ItemCode::BI179,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI180: ItemRow = ItemRow {
    code: ItemCode::BI180,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI181: ItemRow = ItemRow {
    code: ItemCode::BI181,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI182: ItemRow = ItemRow {
    code: ItemCode::BI182,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI183: ItemRow = ItemRow {
    code: ItemCode::BI183,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI184: ItemRow = ItemRow {
    code: ItemCode::BI184,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI185: ItemRow = ItemRow {
    code: ItemCode::BI185,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI186: ItemRow = ItemRow {
    code: ItemCode::BI186,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI187: ItemRow = ItemRow {
    code: ItemCode::BI187,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI188: ItemRow = ItemRow {
    code: ItemCode::BI188,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI189: ItemRow = ItemRow {
    code: ItemCode::BI189,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI190: ItemRow = ItemRow {
    code: ItemCode::BI190,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI191: ItemRow = ItemRow {
    code: ItemCode::BI191,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI192: ItemRow = ItemRow {
    code: ItemCode::BI192,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI193: ItemRow = ItemRow {
    code: ItemCode::BI193,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI195: ItemRow = ItemRow {
    code: ItemCode::BI195,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI196: ItemRow = ItemRow {
    code: ItemCode::BI196,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI197: ItemRow = ItemRow {
    code: ItemCode::BI197,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI198: ItemRow = ItemRow {
    code: ItemCode::BI198,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI201: ItemRow = ItemRow {
    code: ItemCode::BI201,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI202: ItemRow = ItemRow {
    code: ItemCode::BI202,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI203: ItemRow = ItemRow {
    code: ItemCode::BI203,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI204: ItemRow = ItemRow {
    code: ItemCode::BI204,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI205: ItemRow = ItemRow {
    code: ItemCode::BI205,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI206: ItemRow = ItemRow {
    code: ItemCode::BI206,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI207: ItemRow = ItemRow {
    code: ItemCode::BI207,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI208: ItemRow = ItemRow {
    code: ItemCode::BI208,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI209: ItemRow = ItemRow {
    code: ItemCode::BI209,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI210: ItemRow = ItemRow {
    code: ItemCode::BI210,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI211: ItemRow = ItemRow {
    code: ItemCode::BI211,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI212: ItemRow = ItemRow {
    code: ItemCode::BI212,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI213: ItemRow = ItemRow {
    code: ItemCode::BI213,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI214: ItemRow = ItemRow {
    code: ItemCode::BI214,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI215: ItemRow = ItemRow {
    code: ItemCode::BI215,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI216: ItemRow = ItemRow {
    code: ItemCode::BI216,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI217: ItemRow = ItemRow {
    code: ItemCode::BI217,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI218: ItemRow = ItemRow {
    code: ItemCode::BI218,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI219: ItemRow = ItemRow {
    code: ItemCode::BI219,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI220: ItemRow = ItemRow {
    code: ItemCode::BI220,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI221: ItemRow = ItemRow {
    code: ItemCode::BI221,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI222: ItemRow = ItemRow {
    code: ItemCode::BI222,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI223: ItemRow = ItemRow {
    code: ItemCode::BI223,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI224: ItemRow = ItemRow {
    code: ItemCode::BI224,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI225: ItemRow = ItemRow {
    code: ItemCode::BI225,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI226: ItemRow = ItemRow {
    code: ItemCode::BI226,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI227: ItemRow = ItemRow {
    code: ItemCode::BI227,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI228: ItemRow = ItemRow {
    code: ItemCode::BI228,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI229: ItemRow = ItemRow {
    code: ItemCode::BI229,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI230: ItemRow = ItemRow {
    code: ItemCode::BI230,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI231: ItemRow = ItemRow {
    code: ItemCode::BI231,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI232: ItemRow = ItemRow {
    code: ItemCode::BI232,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI233: ItemRow = ItemRow {
    code: ItemCode::BI233,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI234: ItemRow = ItemRow {
    code: ItemCode::BI234,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI235: ItemRow = ItemRow {
    code: ItemCode::BI235,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI236: ItemRow = ItemRow {
    code: ItemCode::BI236,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI237: ItemRow = ItemRow {
    code: ItemCode::BI237,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI238: ItemRow = ItemRow {
    code: ItemCode::BI238,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI239: ItemRow = ItemRow {
    code: ItemCode::BI239,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI240: ItemRow = ItemRow {
    code: ItemCode::BI240,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI241: ItemRow = ItemRow {
    code: ItemCode::BI241,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI242: ItemRow = ItemRow {
    code: ItemCode::BI242,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI243: ItemRow = ItemRow {
    code: ItemCode::BI243,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI244: ItemRow = ItemRow {
    code: ItemCode::BI244,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI245: ItemRow = ItemRow {
    code: ItemCode::BI245,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI246: ItemRow = ItemRow {
    code: ItemCode::BI246,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI247: ItemRow = ItemRow {
    code: ItemCode::BI247,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI248: ItemRow = ItemRow {
    code: ItemCode::BI248,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI249: ItemRow = ItemRow {
    code: ItemCode::BI249,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI250: ItemRow = ItemRow {
    code: ItemCode::BI250,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI251: ItemRow = ItemRow {
    code: ItemCode::BI251,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI252: ItemRow = ItemRow {
    code: ItemCode::BI252,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI253: ItemRow = ItemRow {
    code: ItemCode::BI253,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI254: ItemRow = ItemRow {
    code: ItemCode::BI254,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI255: ItemRow = ItemRow {
    code: ItemCode::BI255,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI256: ItemRow = ItemRow {
    code: ItemCode::BI256,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI257: ItemRow = ItemRow {
    code: ItemCode::BI257,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI258: ItemRow = ItemRow {
    code: ItemCode::BI258,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI259: ItemRow = ItemRow {
    code: ItemCode::BI259,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI260: ItemRow = ItemRow {
    code: ItemCode::BI260,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI261: ItemRow = ItemRow {
    code: ItemCode::BI261,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI262: ItemRow = ItemRow {
    code: ItemCode::BI262,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI266: ItemRow = ItemRow {
    code: ItemCode::BI266,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI267: ItemRow = ItemRow {
    code: ItemCode::BI267,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI268: ItemRow = ItemRow {
    code: ItemCode::BI268,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI269: ItemRow = ItemRow {
    code: ItemCode::BI269,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI270: ItemRow = ItemRow {
    code: ItemCode::BI270,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR101: ItemRow = ItemRow {
    code: ItemCode::RR101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR102: ItemRow = ItemRow {
    code: ItemCode::RR102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR103: ItemRow = ItemRow {
    code: ItemCode::RR103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR104: ItemRow = ItemRow {
    code: ItemCode::RR104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR105: ItemRow = ItemRow {
    code: ItemCode::RR105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR106: ItemRow = ItemRow {
    code: ItemCode::RR106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR107: ItemRow = ItemRow {
    code: ItemCode::RR107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR108: ItemRow = ItemRow {
    code: ItemCode::RR108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR109: ItemRow = ItemRow {
    code: ItemCode::RR109,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR110: ItemRow = ItemRow {
    code: ItemCode::RR110,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR111: ItemRow = ItemRow {
    code: ItemCode::RR111,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_RR112: ItemRow = ItemRow {
    code: ItemCode::RR112,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV101: ItemRow = ItemRow {
    code: ItemCode::EV101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV102: ItemRow = ItemRow {
    code: ItemCode::EV102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV103: ItemRow = ItemRow {
    code: ItemCode::EV103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV104: ItemRow = ItemRow {
    code: ItemCode::EV104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV105: ItemRow = ItemRow {
    code: ItemCode::EV105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV106: ItemRow = ItemRow {
    code: ItemCode::EV106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV107: ItemRow = ItemRow {
    code: ItemCode::EV107,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV108: ItemRow = ItemRow {
    code: ItemCode::EV108,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV201: ItemRow = ItemRow {
    code: ItemCode::EV201,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_EV202: ItemRow = ItemRow {
    code: ItemCode::EV202,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ101: ItemRow = ItemRow {
    code: ItemCode::PZ101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ102: ItemRow = ItemRow {
    code: ItemCode::PZ102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ103: ItemRow = ItemRow {
    code: ItemCode::PZ103,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ104: ItemRow = ItemRow {
    code: ItemCode::PZ104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ105: ItemRow = ItemRow {
    code: ItemCode::PZ105,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ106: ItemRow = ItemRow {
    code: ItemCode::PZ106,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ107: ItemRow = ItemRow {
    code: ItemCode::PZ107,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ108: ItemRow = ItemRow {
    code: ItemCode::PZ108,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ201: ItemRow = ItemRow {
    code: ItemCode::PZ201,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ202: ItemRow = ItemRow {
    code: ItemCode::PZ202,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ203: ItemRow = ItemRow {
    code: ItemCode::PZ203,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ204: ItemRow = ItemRow {
    code: ItemCode::PZ204,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ205: ItemRow = ItemRow {
    code: ItemCode::PZ205,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ206: ItemRow = ItemRow {
    code: ItemCode::PZ206,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ207: ItemRow = ItemRow {
    code: ItemCode::PZ207,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PZ208: ItemRow = ItemRow {
    code: ItemCode::PZ208,
    w: ITEMSIZE * 2,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_CH101: ItemRow = ItemRow {
    code: ItemCode::CH101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_CH102: ItemRow = ItemRow {
    code: ItemCode::CH102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_CH103: ItemRow = ItemRow {
    code: ItemCode::CH103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_CH104: ItemRow = ItemRow {
    code: ItemCode::CH104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SE101: ItemRow = ItemRow {
    code: ItemCode::SE101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SE102: ItemRow = ItemRow {
    code: ItemCode::SE102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SE103: ItemRow = ItemRow {
    code: ItemCode::SE103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SE104: ItemRow = ItemRow {
    code: ItemCode::SE104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_SE105: ItemRow = ItemRow {
    code: ItemCode::SE105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR101: ItemRow = ItemRow {
    code: ItemCode::PR101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR102: ItemRow = ItemRow {
    code: ItemCode::PR102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR103: ItemRow = ItemRow {
    code: ItemCode::PR103,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR104: ItemRow = ItemRow {
    code: ItemCode::PR104,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR105: ItemRow = ItemRow {
    code: ItemCode::PR105,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR106: ItemRow = ItemRow {
    code: ItemCode::PR106,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR107: ItemRow = ItemRow {
    code: ItemCode::PR107,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR108: ItemRow = ItemRow {
    code: ItemCode::PR108,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR120: ItemRow = ItemRow {
    code: ItemCode::PR120,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR121: ItemRow = ItemRow {
    code: ItemCode::PR121,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR122: ItemRow = ItemRow {
    code: ItemCode::PR122,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR123: ItemRow = ItemRow {
    code: ItemCode::PR123,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR124: ItemRow = ItemRow {
    code: ItemCode::PR124,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR125: ItemRow = ItemRow {
    code: ItemCode::PR125,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR126: ItemRow = ItemRow {
    code: ItemCode::PR126,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR127: ItemRow = ItemRow {
    code: ItemCode::PR127,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR128: ItemRow = ItemRow {
    code: ItemCode::PR128,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR129: ItemRow = ItemRow {
    code: ItemCode::PR129,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR130: ItemRow = ItemRow {
    code: ItemCode::PR130,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR201: ItemRow = ItemRow {
    code: ItemCode::PR201,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR202: ItemRow = ItemRow {
    code: ItemCode::PR202,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR203: ItemRow = ItemRow {
    code: ItemCode::PR203,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR204: ItemRow = ItemRow {
    code: ItemCode::PR204,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR205: ItemRow = ItemRow {
    code: ItemCode::PR205,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR206: ItemRow = ItemRow {
    code: ItemCode::PR206,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR207: ItemRow = ItemRow {
    code: ItemCode::PR207,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR208: ItemRow = ItemRow {
    code: ItemCode::PR208,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR209: ItemRow = ItemRow {
    code: ItemCode::PR209,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR210: ItemRow = ItemRow {
    code: ItemCode::PR210,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR211: ItemRow = ItemRow {
    code: ItemCode::PR211,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR212: ItemRow = ItemRow {
    code: ItemCode::PR212,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR213: ItemRow = ItemRow {
    code: ItemCode::PR213,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR214: ItemRow = ItemRow {
    code: ItemCode::PR214,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR301: ItemRow = ItemRow {
    code: ItemCode::PR301,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR302: ItemRow = ItemRow {
    code: ItemCode::PR302,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR303: ItemRow = ItemRow {
    code: ItemCode::PR303,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR304: ItemRow = ItemRow {
    code: ItemCode::PR304,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR305: ItemRow = ItemRow {
    code: ItemCode::PR305,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR306: ItemRow = ItemRow {
    code: ItemCode::PR306,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR307: ItemRow = ItemRow {
    code: ItemCode::PR307,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR308: ItemRow = ItemRow {
    code: ItemCode::PR308,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR309: ItemRow = ItemRow {
    code: ItemCode::PR309,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR310: ItemRow = ItemRow {
    code: ItemCode::PR310,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR311: ItemRow = ItemRow {
    code: ItemCode::PR311,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR312: ItemRow = ItemRow {
    code: ItemCode::PR312,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR313: ItemRow = ItemRow {
    code: ItemCode::PR313,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR314: ItemRow = ItemRow {
    code: ItemCode::PR314,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR401: ItemRow = ItemRow {
    code: ItemCode::PR401,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR402: ItemRow = ItemRow {
    code: ItemCode::PR402,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR403: ItemRow = ItemRow {
    code: ItemCode::PR403,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR404: ItemRow = ItemRow {
    code: ItemCode::PR404,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR405: ItemRow = ItemRow {
    code: ItemCode::PR405,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR406: ItemRow = ItemRow {
    code: ItemCode::PR406,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR407: ItemRow = ItemRow {
    code: ItemCode::PR407,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR408: ItemRow = ItemRow {
    code: ItemCode::PR408,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR409: ItemRow = ItemRow {
    code: ItemCode::PR409,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR410: ItemRow = ItemRow {
    code: ItemCode::PR410,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR411: ItemRow = ItemRow {
    code: ItemCode::PR411,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR412: ItemRow = ItemRow {
    code: ItemCode::PR412,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR413: ItemRow = ItemRow {
    code: ItemCode::PR413,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_PR414: ItemRow = ItemRow {
    code: ItemCode::PR414,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Seel,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR101: ItemRow = ItemRow {
    code: ItemCode::WR101,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR102: ItemRow = ItemRow {
    code: ItemCode::WR102,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR103: ItemRow = ItemRow {
    code: ItemCode::WR103,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR104: ItemRow = ItemRow {
    code: ItemCode::WR104,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR105: ItemRow = ItemRow {
    code: ItemCode::WR105,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR106: ItemRow = ItemRow {
    code: ItemCode::WR106,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR107: ItemRow = ItemRow {
    code: ItemCode::WR107,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR108: ItemRow = ItemRow {
    code: ItemCode::WR108,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WR109: ItemRow = ItemRow {
    code: ItemCode::WR109,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR101: ItemRow = ItemRow {
    code: ItemCode::DR101,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR102: ItemRow = ItemRow {
    code: ItemCode::DR102,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR103: ItemRow = ItemRow {
    code: ItemCode::DR103,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR104: ItemRow = ItemRow {
    code: ItemCode::DR104,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR105: ItemRow = ItemRow {
    code: ItemCode::DR105,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR106: ItemRow = ItemRow {
    code: ItemCode::DR106,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR107: ItemRow = ItemRow {
    code: ItemCode::DR107,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR108: ItemRow = ItemRow {
    code: ItemCode::DR108,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR109: ItemRow = ItemRow {
    code: ItemCode::DR109,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR110: ItemRow = ItemRow {
    code: ItemCode::DR110,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DR111: ItemRow = ItemRow {
    code: ItemCode::DR111,
    w: ITEMSIZE,
    h: ITEMSIZE * 2,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_CA131: ItemRow = ItemRow {
    code: ItemCode::CA131,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA133: ItemRow = ItemRow {
    code: ItemCode::CA133,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA135: ItemRow = ItemRow {
    code: ItemCode::CA135,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA137: ItemRow = ItemRow {
    code: ItemCode::CA137,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA143: ItemRow = ItemRow {
    code: ItemCode::CA143,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA145: ItemRow = ItemRow {
    code: ItemCode::CA145,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA154: ItemRow = ItemRow {
    code: ItemCode::CA154,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA155: ItemRow = ItemRow {
    code: ItemCode::CA155,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA166: ItemRow = ItemRow {
    code: ItemCode::CA166,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA167: ItemRow = ItemRow {
    code: ItemCode::CA167,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA168: ItemRow = ItemRow {
    code: ItemCode::CA168,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA169: ItemRow = ItemRow {
    code: ItemCode::CA169,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA185: ItemRow = ItemRow {
    code: ItemCode::CA185,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA186: ItemRow = ItemRow {
    code: ItemCode::CA186,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA187: ItemRow = ItemRow {
    code: ItemCode::CA187,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA188: ItemRow = ItemRow {
    code: ItemCode::CA188,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA189: ItemRow = ItemRow {
    code: ItemCode::CA189,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA190: ItemRow = ItemRow {
    code: ItemCode::CA190,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA191: ItemRow = ItemRow {
    code: ItemCode::CA191,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA192: ItemRow = ItemRow {
    code: ItemCode::CA192,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA193: ItemRow = ItemRow {
    code: ItemCode::CA193,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA194: ItemRow = ItemRow {
    code: ItemCode::CA194,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA195: ItemRow = ItemRow {
    code: ItemCode::CA195,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA196: ItemRow = ItemRow {
    code: ItemCode::CA196,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA232: ItemRow = ItemRow {
    code: ItemCode::CA232,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA234: ItemRow = ItemRow {
    code: ItemCode::CA234,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA235: ItemRow = ItemRow {
    code: ItemCode::CA235,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA237: ItemRow = ItemRow {
    code: ItemCode::CA237,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA243: ItemRow = ItemRow {
    code: ItemCode::CA243,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA245: ItemRow = ItemRow {
    code: ItemCode::CA245,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA254: ItemRow = ItemRow {
    code: ItemCode::CA254,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA255: ItemRow = ItemRow {
    code: ItemCode::CA255,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA266: ItemRow = ItemRow {
    code: ItemCode::CA266,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA267: ItemRow = ItemRow {
    code: ItemCode::CA267,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA268: ItemRow = ItemRow {
    code: ItemCode::CA268,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA269: ItemRow = ItemRow {
    code: ItemCode::CA269,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA285: ItemRow = ItemRow {
    code: ItemCode::CA285,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA286: ItemRow = ItemRow {
    code: ItemCode::CA286,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA287: ItemRow = ItemRow {
    code: ItemCode::CA287,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA288: ItemRow = ItemRow {
    code: ItemCode::CA288,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA289: ItemRow = ItemRow {
    code: ItemCode::CA289,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA290: ItemRow = ItemRow {
    code: ItemCode::CA290,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA291: ItemRow = ItemRow {
    code: ItemCode::CA291,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA292: ItemRow = ItemRow {
    code: ItemCode::CA292,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA293: ItemRow = ItemRow {
    code: ItemCode::CA293,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA294: ItemRow = ItemRow {
    code: ItemCode::CA294,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA295: ItemRow = ItemRow {
    code: ItemCode::CA295,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA296: ItemRow = ItemRow {
    code: ItemCode::CA296,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA501: ItemRow = ItemRow {
    code: ItemCode::CA501,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA502: ItemRow = ItemRow {
    code: ItemCode::CA502,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA503: ItemRow = ItemRow {
    code: ItemCode::CA503,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA504: ItemRow = ItemRow {
    code: ItemCode::CA504,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA505: ItemRow = ItemRow {
    code: ItemCode::CA505,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA506: ItemRow = ItemRow {
    code: ItemCode::CA506,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA507: ItemRow = ItemRow {
    code: ItemCode::CA507,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA508: ItemRow = ItemRow {
    code: ItemCode::CA508,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA509: ItemRow = ItemRow {
    code: ItemCode::CA509,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA510: ItemRow = ItemRow {
    code: ItemCode::CA510,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA511: ItemRow = ItemRow {
    code: ItemCode::CA511,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA512: ItemRow = ItemRow {
    code: ItemCode::CA512,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA515: ItemRow = ItemRow {
    code: ItemCode::CA515,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA516: ItemRow = ItemRow {
    code: ItemCode::CA516,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA517: ItemRow = ItemRow {
    code: ItemCode::CA517,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA518: ItemRow = ItemRow {
    code: ItemCode::CA518,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA519: ItemRow = ItemRow {
    code: ItemCode::CA519,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA520: ItemRow = ItemRow {
    code: ItemCode::CA520,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA521: ItemRow = ItemRow {
    code: ItemCode::CA521,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA522: ItemRow = ItemRow {
    code: ItemCode::CA522,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA523: ItemRow = ItemRow {
    code: ItemCode::CA523,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA524: ItemRow = ItemRow {
    code: ItemCode::CA524,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA525: ItemRow = ItemRow {
    code: ItemCode::CA525,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA526: ItemRow = ItemRow {
    code: ItemCode::CA526,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA527: ItemRow = ItemRow {
    code: ItemCode::CA527,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA528: ItemRow = ItemRow {
    code: ItemCode::CA528,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA531: ItemRow = ItemRow {
    code: ItemCode::CA531,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA532: ItemRow = ItemRow {
    code: ItemCode::CA532,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA533: ItemRow = ItemRow {
    code: ItemCode::CA533,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA534: ItemRow = ItemRow {
    code: ItemCode::CA534,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA535: ItemRow = ItemRow {
    code: ItemCode::CA535,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA536: ItemRow = ItemRow {
    code: ItemCode::CA536,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA537: ItemRow = ItemRow {
    code: ItemCode::CA537,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA538: ItemRow = ItemRow {
    code: ItemCode::CA538,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA539: ItemRow = ItemRow {
    code: ItemCode::CA539,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA540: ItemRow = ItemRow {
    code: ItemCode::CA540,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA541: ItemRow = ItemRow {
    code: ItemCode::CA541,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA542: ItemRow = ItemRow {
    code: ItemCode::CA542,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA543: ItemRow = ItemRow {
    code: ItemCode::CA543,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA544: ItemRow = ItemRow {
    code: ItemCode::CA544,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA545: ItemRow = ItemRow {
    code: ItemCode::CA545,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA546: ItemRow = ItemRow {
    code: ItemCode::CA546,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA547: ItemRow = ItemRow {
    code: ItemCode::CA547,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA548: ItemRow = ItemRow {
    code: ItemCode::CA548,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA601: ItemRow = ItemRow {
    code: ItemCode::CA601,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA602: ItemRow = ItemRow {
    code: ItemCode::CA602,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA603: ItemRow = ItemRow {
    code: ItemCode::CA603,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA604: ItemRow = ItemRow {
    code: ItemCode::CA604,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA605: ItemRow = ItemRow {
    code: ItemCode::CA605,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA606: ItemRow = ItemRow {
    code: ItemCode::CA606,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA607: ItemRow = ItemRow {
    code: ItemCode::CA607,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA608: ItemRow = ItemRow {
    code: ItemCode::CA608,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA609: ItemRow = ItemRow {
    code: ItemCode::CA609,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA610: ItemRow = ItemRow {
    code: ItemCode::CA610,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA611: ItemRow = ItemRow {
    code: ItemCode::CA611,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA612: ItemRow = ItemRow {
    code: ItemCode::CA612,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA615: ItemRow = ItemRow {
    code: ItemCode::CA615,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA616: ItemRow = ItemRow {
    code: ItemCode::CA616,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA617: ItemRow = ItemRow {
    code: ItemCode::CA617,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA618: ItemRow = ItemRow {
    code: ItemCode::CA618,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA619: ItemRow = ItemRow {
    code: ItemCode::CA619,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA620: ItemRow = ItemRow {
    code: ItemCode::CA620,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA621: ItemRow = ItemRow {
    code: ItemCode::CA621,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA622: ItemRow = ItemRow {
    code: ItemCode::CA622,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA623: ItemRow = ItemRow {
    code: ItemCode::CA623,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA624: ItemRow = ItemRow {
    code: ItemCode::CA624,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA625: ItemRow = ItemRow {
    code: ItemCode::CA625,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA626: ItemRow = ItemRow {
    code: ItemCode::CA626,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA627: ItemRow = ItemRow {
    code: ItemCode::CA627,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA628: ItemRow = ItemRow {
    code: ItemCode::CA628,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA631: ItemRow = ItemRow {
    code: ItemCode::CA631,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA632: ItemRow = ItemRow {
    code: ItemCode::CA632,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA633: ItemRow = ItemRow {
    code: ItemCode::CA633,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA634: ItemRow = ItemRow {
    code: ItemCode::CA634,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA635: ItemRow = ItemRow {
    code: ItemCode::CA635,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA636: ItemRow = ItemRow {
    code: ItemCode::CA636,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA637: ItemRow = ItemRow {
    code: ItemCode::CA637,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA638: ItemRow = ItemRow {
    code: ItemCode::CA638,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA639: ItemRow = ItemRow {
    code: ItemCode::CA639,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA640: ItemRow = ItemRow {
    code: ItemCode::CA640,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA641: ItemRow = ItemRow {
    code: ItemCode::CA641,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA642: ItemRow = ItemRow {
    code: ItemCode::CA642,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA643: ItemRow = ItemRow {
    code: ItemCode::CA643,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA644: ItemRow = ItemRow {
    code: ItemCode::CA644,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA645: ItemRow = ItemRow {
    code: ItemCode::CA645,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA646: ItemRow = ItemRow {
    code: ItemCode::CA646,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA647: ItemRow = ItemRow {
    code: ItemCode::CA647,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA648: ItemRow = ItemRow {
    code: ItemCode::CA648,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA649: ItemRow = ItemRow {
    code: ItemCode::CA649,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA549: ItemRow = ItemRow {
    code: ItemCode::CA549,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA650: ItemRow = ItemRow {
    code: ItemCode::CA650,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA550: ItemRow = ItemRow {
    code: ItemCode::CA550,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA651: ItemRow = ItemRow {
    code: ItemCode::CA651,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA551: ItemRow = ItemRow {
    code: ItemCode::CA551,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA652: ItemRow = ItemRow {
    code: ItemCode::CA652,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA552: ItemRow = ItemRow {
    code: ItemCode::CA552,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA653: ItemRow = ItemRow {
    code: ItemCode::CA653,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA553: ItemRow = ItemRow {
    code: ItemCode::CA553,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA654: ItemRow = ItemRow {
    code: ItemCode::CA654,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA554: ItemRow = ItemRow {
    code: ItemCode::CA554,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA655: ItemRow = ItemRow {
    code: ItemCode::CA655,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA555: ItemRow = ItemRow {
    code: ItemCode::CA555,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA656: ItemRow = ItemRow {
    code: ItemCode::CA656,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA556: ItemRow = ItemRow {
    code: ItemCode::CA556,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA657: ItemRow = ItemRow {
    code: ItemCode::CA657,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA557: ItemRow = ItemRow {
    code: ItemCode::CA557,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA658: ItemRow = ItemRow {
    code: ItemCode::CA658,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA558: ItemRow = ItemRow {
    code: ItemCode::CA558,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA659: ItemRow = ItemRow {
    code: ItemCode::CA659,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA559: ItemRow = ItemRow {
    code: ItemCode::CA559,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA660: ItemRow = ItemRow {
    code: ItemCode::CA660,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA560: ItemRow = ItemRow {
    code: ItemCode::CA560,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA661: ItemRow = ItemRow {
    code: ItemCode::CA661,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA561: ItemRow = ItemRow {
    code: ItemCode::CA561,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA662: ItemRow = ItemRow {
    code: ItemCode::CA662,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA562: ItemRow = ItemRow {
    code: ItemCode::CA562,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA663: ItemRow = ItemRow {
    code: ItemCode::CA663,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA563: ItemRow = ItemRow {
    code: ItemCode::CA563,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA664: ItemRow = ItemRow {
    code: ItemCode::CA664,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_CA564: ItemRow = ItemRow {
    code: ItemCode::CA564,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Costume,
    model_posi: Some(InventoryPosition::COSTUME),
    weapon_class: None,
};

static ITEM_DB144: ItemRow = ItemRow {
    code: ItemCode::DB144,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DG131: ItemRow = ItemRow {
    code: ItemCode::DG131,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA230: ItemRow = ItemRow {
    code: ItemCode::OA230,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DA159: ItemRow = ItemRow {
    code: ItemCode::DA159,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA161: ItemRow = ItemRow {
    code: ItemCode::DA161,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_DA165: ItemRow = ItemRow {
    code: ItemCode::DA165,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 4,
    class: ItemClass::Armor,
    model_posi: Some(InventoryPosition::ARMOR),
    weapon_class: None,
};

static ITEM_GG101: ItemRow = ItemRow {
    code: ItemCode::GG101,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GG102: ItemRow = ItemRow {
    code: ItemCode::GG102,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS118: ItemRow = ItemRow {
    code: ItemCode::OS118,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS119: ItemRow = ItemRow {
    code: ItemCode::OS119,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OS120: ItemRow = ItemRow {
    code: ItemCode::OS120,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Sheltom,
    model_posi: None,
    weapon_class: None,
};

static ITEM_GG103: ItemRow = ItemRow {
    code: ItemCode::GG103,
    w: 240,
    h: 160,
    class: ItemClass::Quest,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI301: ItemRow = ItemRow {
    code: ItemCode::BI301,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI302: ItemRow = ItemRow {
    code: ItemCode::BI302,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI303: ItemRow = ItemRow {
    code: ItemCode::BI303,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI304: ItemRow = ItemRow {
    code: ItemCode::BI304,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI305: ItemRow = ItemRow {
    code: ItemCode::BI305,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI306: ItemRow = ItemRow {
    code: ItemCode::BI306,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI307: ItemRow = ItemRow {
    code: ItemCode::BI307,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI308: ItemRow = ItemRow {
    code: ItemCode::BI308,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI309: ItemRow = ItemRow {
    code: ItemCode::BI309,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI310: ItemRow = ItemRow {
    code: ItemCode::BI310,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI311: ItemRow = ItemRow {
    code: ItemCode::BI311,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI312: ItemRow = ItemRow {
    code: ItemCode::BI312,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI313: ItemRow = ItemRow {
    code: ItemCode::BI313,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI314: ItemRow = ItemRow {
    code: ItemCode::BI314,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI315: ItemRow = ItemRow {
    code: ItemCode::BI315,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI316: ItemRow = ItemRow {
    code: ItemCode::BI316,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI317: ItemRow = ItemRow {
    code: ItemCode::BI317,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI318: ItemRow = ItemRow {
    code: ItemCode::BI318,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI319: ItemRow = ItemRow {
    code: ItemCode::BI319,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI320: ItemRow = ItemRow {
    code: ItemCode::BI320,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI321: ItemRow = ItemRow {
    code: ItemCode::BI321,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI322: ItemRow = ItemRow {
    code: ItemCode::BI322,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI323: ItemRow = ItemRow {
    code: ItemCode::BI323,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI324: ItemRow = ItemRow {
    code: ItemCode::BI324,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI325: ItemRow = ItemRow {
    code: ItemCode::BI325,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI326: ItemRow = ItemRow {
    code: ItemCode::BI326,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI330: ItemRow = ItemRow {
    code: ItemCode::BI330,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI331: ItemRow = ItemRow {
    code: ItemCode::BI331,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI332: ItemRow = ItemRow {
    code: ItemCode::BI332,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI333: ItemRow = ItemRow {
    code: ItemCode::BI333,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI334: ItemRow = ItemRow {
    code: ItemCode::BI334,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI335: ItemRow = ItemRow {
    code: ItemCode::BI335,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI336: ItemRow = ItemRow {
    code: ItemCode::BI336,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI340: ItemRow = ItemRow {
    code: ItemCode::BI340,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI341: ItemRow = ItemRow {
    code: ItemCode::BI341,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI342: ItemRow = ItemRow {
    code: ItemCode::BI342,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI343: ItemRow = ItemRow {
    code: ItemCode::BI343,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI344: ItemRow = ItemRow {
    code: ItemCode::BI344,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI345: ItemRow = ItemRow {
    code: ItemCode::BI345,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI380: ItemRow = ItemRow {
    code: ItemCode::BI380,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI381: ItemRow = ItemRow {
    code: ItemCode::BI381,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI382: ItemRow = ItemRow {
    code: ItemCode::BI382,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI383: ItemRow = ItemRow {
    code: ItemCode::BI383,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI384: ItemRow = ItemRow {
    code: ItemCode::BI384,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI385: ItemRow = ItemRow {
    code: ItemCode::BI385,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI386: ItemRow = ItemRow {
    code: ItemCode::BI386,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI387: ItemRow = ItemRow {
    code: ItemCode::BI387,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI388: ItemRow = ItemRow {
    code: ItemCode::BI388,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI389: ItemRow = ItemRow {
    code: ItemCode::BI389,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI390: ItemRow = ItemRow {
    code: ItemCode::BI390,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI391: ItemRow = ItemRow {
    code: ItemCode::BI391,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI392: ItemRow = ItemRow {
    code: ItemCode::BI392,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI393: ItemRow = ItemRow {
    code: ItemCode::BI393,
    w: ITEMSIZE,
    h: ITEMSIZE,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_WA186: ItemRow = ItemRow {
    code: ItemCode::WA186,
    w: 44,
    h: 88,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WA188: ItemRow = ItemRow {
    code: ItemCode::WA188,
    w: 44,
    h: 88,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WC186: ItemRow = ItemRow {
    code: ItemCode::WC186,
    w: 44,
    h: ITEMSIZE * 3,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH186: ItemRow = ItemRow {
    code: ItemCode::WH186,
    w: 44,
    h: 88,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WH188: ItemRow = ItemRow {
    code: ItemCode::WH188,
    w: 44,
    h: 88,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WP186: ItemRow = ItemRow {
    code: ItemCode::WP186,
    w: 44,
    h: 88,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS186: ItemRow = ItemRow {
    code: ItemCode::WS186,
    w: 44,
    h: 88,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_WS286: ItemRow = ItemRow {
    code: ItemCode::WS286,
    w: 44,
    h: 88,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WS288: ItemRow = ItemRow {
    code: ItemCode::WS288,
    w: 44,
    h: 88,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WM186: ItemRow = ItemRow {
    code: ItemCode::WM186,
    w: 44,
    h: 88,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WM188: ItemRow = ItemRow {
    code: ItemCode::WM188,
    w: 44,
    h: 88,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WD186: ItemRow = ItemRow {
    code: ItemCode::WD186,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WN186: ItemRow = ItemRow {
    code: ItemCode::WN186,
    w: ITEMSIZE * 3,
    h: ITEMSIZE * 2,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Casting),
};

static ITEM_WV186: ItemRow = ItemRow {
    code: ItemCode::WV186,
    w: ITEMSIZE * 2,
    h: ITEMSIZE * 4,
    class: ItemClass::WeaponTwo,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::NotShooting),
};

static ITEM_WT186: ItemRow = ItemRow {
    code: ItemCode::WT186,
    w: 22,
    h: 88,
    class: ItemClass::WeaponOne,
    model_posi: Some(InventoryPosition::RIGHT_HAND),
    weapon_class: Some(ItemWeaponClass::Shooting),
};

static ITEM_DG186: ItemRow = ItemRow {
    code: ItemCode::DG186,
    w: 44,
    h: ITEMSIZE * 2,
    class: ItemClass::Gloves,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DB186: ItemRow = ItemRow {
    code: ItemCode::DB186,
    w: 44,
    h: ITEMSIZE * 2,
    class: ItemClass::Boots,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA286: ItemRow = ItemRow {
    code: ItemCode::OA286,
    w: 44,
    h: ITEMSIZE * 2,
    class: ItemClass::Armlet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_DS186: ItemRow = ItemRow {
    code: ItemCode::DS186,
    w: 44,
    h: 88,
    class: ItemClass::Shield,
    model_posi: Some(InventoryPosition::LEFT_HAND),
    weapon_class: None,
};

static ITEM_OE101: ItemRow = ItemRow {
    code: ItemCode::OE101,
    w: 22,
    h: 22,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OE102: ItemRow = ItemRow {
    code: ItemCode::OE102,
    w: 22,
    h: 22,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OE103: ItemRow = ItemRow {
    code: ItemCode::OE103,
    w: 22,
    h: 22,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OE104: ItemRow = ItemRow {
    code: ItemCode::OE104,
    w: 22,
    h: 22,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OE105: ItemRow = ItemRow {
    code: ItemCode::OE105,
    w: 22,
    h: 22,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OE106: ItemRow = ItemRow {
    code: ItemCode::OE106,
    w: 22,
    h: 22,
    class: ItemClass::Ring,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA151: ItemRow = ItemRow {
    code: ItemCode::OA151,
    w: 22,
    h: 22,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_OA153: ItemRow = ItemRow {
    code: ItemCode::OA153,
    w: 22,
    h: 22,
    class: ItemClass::Amulet,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI263: ItemRow = ItemRow {
    code: ItemCode::BI263,
    w: 44,
    h: 22,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI264: ItemRow = ItemRow {
    code: ItemCode::BI264,
    w: 44,
    h: 22,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};

static ITEM_BI265: ItemRow = ItemRow {
    code: ItemCode::BI265,
    w: 44,
    h: 22,
    class: ItemClass::Ecore,
    model_posi: None,
    weapon_class: None,
};
