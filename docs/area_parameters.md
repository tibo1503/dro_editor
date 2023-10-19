# DRO Editor areas.yaml
## Parameters
| Parameters                  | Optional | Type | Restrictions | Default | Description |
|-----------------------------|----------|--------------------------------------|---------------------------------------------|-------------------------------------------|--|
| area                        | No       | String                               | Verify if another area exist with this name |                                           | Name of the area. |
| background                  | No       | Background                           | Verify if exist                             |                                           | Default background of the area. |
| *afk_delay                   | Yes      | Time                                 |                                             | 0                                         | If a positive number, the number of seconds that must pass to mark an inactive player as AFK and kick them to `afk_sendto`. If 0, no such AFK kicks take place. |
| *afk_sendto                  | Yes      | Area                                 | Verify if exist                             | 0                                         | The ID of the area to kick an AFK player to. |
| *background_tod              | Yes      | Background                           |                                             | NONE                                      | Backgrounds per time of day. |
| *bglock                      | Yes      | Bool                                 |                                             | False                                     | If false, only moderators can lock the area background; if true, no one can lock it. |
| *bullet                      | Yes      | Bool                                 |                                             | True                                      | If false, only GM+ can send IC messages with bullets/shouts. If true, no such restriction is imposed. |
| *cbg_allowed                 | Yes      | Bool                                 |                                             | False                                     | If false, only GM+ can set the area's background to one not in the server's list. If true, no such restriction is imposed. |
| *change_reachability_allowed | Yes      | Bool                                 |                                             | True                                      | If false, only GM+ may /unilock or /bilock passages affecting this area. If true, no such restriction is imposed. |
| *default_description         | Yes      | String                               |                                             | `default_area_description` in config.yaml | Output of /look in the area if no custom description is in place via /look_set |
| *evidence_mod                | Yes      | WTH you are ?!                       |                                             | FFA                                       | ALED |
| *gm_iclock_allowed           | Yes      | Bool                                 |                                             | True                                      | If false, only CM+ may /iclock the area. If true, GMs may also /iclock. |
| *iniswap_allowed             | Yes      | Bool                                 |                                             | True                                      | If true, everyone can send IC messages while iniswapped/iniedited; if false, no one can send IC messages while iniswapped/iniedited. |
| *global_allowed              | Yes      | Bool                                 |                                             | True                                      | If false, only CM+ may use /g in the area. If true, anyone can use it. |
| *has_lights                  | Yes      | Bool                                 |                                             | False                                     | If false, the area cannot have its lights turned off via /light or similar. If true, no such restriction is imposed. |
| *lobby_area                  | Yes      | Bool                                 |                                             | False                                     | If true, the area is marked as lobby (which disallows non-GMs+ from sneaking or /knock'ing from or to the area). If false, no such restriction is placed. |
| *locking_allowed             | Yes      | Bool                                 |                                             | False                                     | If false, everyone can lock the area; if true, no one can lock it. |
| *private_area                | Yes      | Bool                                 |                                             | False                                     | If true, the area is marked as private (which disallows everyone from sneaking and GMs+ reading /whisper messages). |
| *reachable_areas             | Yes      | List\<Area> or ALL                    | Verify areas exist                          | <All>                                     | If <ALL>, a passage will be created from this area to every other area. If it is a comma-separated list of areas (by name), a passage will be created from this area only to the listed areas. |
| *restricted_chars            | Yes      | List\<Character>                      | Verify characters exist                     | NONE/NO RESTRICTED CHARACTERS             | If a list of folder names, it prevents non-GMs using said characters from joining the area, and prompts them to choose another one if they are kicked to  |
| *rollp_allowed               | Yes      | Bool                                 |                                             | True                                      | If false, only GM+ may /rollp. If true, everyone can /rollp. |
| *rp_getarea_allowed          | Yes      | Bool                                 |                                             | True                                      | If false, if the server is in RP mode, only GM+ can /getarea and similar. If true, no such restriction is imposed. |
| *rp_getareas_allowed         | Yes      | Bool                                 |                                             | True                                      | If false, if the server is in RP mode, only GM+ can /getareas and similar. If true, no such restriction is imposed. |
| *scream_range                | Yes      | List\<Area> or ALL or REACHABLE_AREAS | Verify areas exist                          | NO AREAS OTHER THAN THIS ONE              | If <ALL>, all areas can receive a /scream message sent from this area. If <REACHABLE_AREAS>, all areas defined in reachable_areas can receive a /scream message sent from this area. If it is a comma-separated list of areas (by name), only said areas can receive /scream messages. |
| *song_switch_allowed         | Yes      | Bool                                 |                                             | False                                     | If false, only GM+ can do /play. If true, no such restriction is imposed. |
| *visible_areas               | Yes      | List\<Area> or ALL or REACHABLE_AREAS | Verify areas exist                          | <REACHABLE_AREAS>                         | If <ALL>, a passage will be set as visible (but not necessarily reachable) from this area to every other area. If <REACHABLE_AREAS>, the procedure will be done to exactly all areas described in the areas' `reachable_areas` key. If it is a comma-separated list of areas (by name), the procedure will be done to the listed areas. |

## Types

| Name       | Possible value     | Description                                     | Possible problem            |
|------------|--------------------|-------------------------------------------------|-----------------------------|
| Bool       | "true" or "false"  | Used to answer to a question                    |                             |
| String     | "ALED"             | A sentence with letter, number, space and other | No ASCII character possibly |
| Area       | "Iris bedroom 404" | Any existing name of Area                       | Bad Area name               |
| Background | "Class Room 1_HD"  | Any existing name of Background                 | Bad background name         |
| Time       | "60"               | Unsigned integer                                | Not positive or beyond 360  |