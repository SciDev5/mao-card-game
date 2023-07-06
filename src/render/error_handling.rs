/* Created by SciDev
 * 
 * This file is responsible for showing a funny error message when
 * a rendering error occurrs.
 */

use rand::seq::SliceRandom;
use crate::render::engine::RenderError;

pub fn print_render_error(err: RenderError) {
    println!("\n\n\n\n{}\n\nERROR:\n{err}",random_error_splash());
}
fn random_error_splash() -> &'static str {
  [r#":: CRASH SPLASH 0 ::
⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⠋⠄⠁⠍⠋⠉⠅⠀⠀⠄⠆⠀⠅⠁⠁⡯⡿⡯⡿⡯⡿⡯⠿⠏⠋⠉⠅⠅
⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⠏⠀⠁⠀⠀⠀⠁⠀⠁⠁⠁⠀⠁⠁⠁⠀⠾⠯⠟⠋⠋⠁⠅⠀⠄⠁⠁⠀⠁⠀
⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⡯⡿⠏⠀⠁⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠆⠇⡇⠆⠄⠁⠁⠁⠀⠁⠁⠀⠀⠀⠀⠀
⠟⠏⠟⠏⠏⠋⠋⠋⠋⠋⠋⠉⠉⠉⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠅⠇⠁⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⡬
⠁⠁⠁⠁⠁⠁⠁⠁⠁⠀⠁⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠁⠇⠄⡆⠁⠇⠆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡯
⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⡄⡯⡏⡇⡆⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠇⠃⠃⠁⠀⠀⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⡿⡯
⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⡤⡏⡯⡏⡯⡏⠇⠀⠁⠀⠀⠀⠀⠀⠄⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡯⡿⡯
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡌⡯⡯⡯⠏⡁⡄⡄⡆⡄⠀⠁⠀⠀⠁⠁⠀⠅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⡿⡯⡿⡯
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡯⡯⡏⡏⡏⡏⡏⠏⠇⡇⠅⡆⡄⡄⡄⡀⠀⠀⠀⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⡾⡯⡿⡯⡿⡯
⠀⠀⠀⠀⠀⠀⡀⡀⡄⡄⠭⡯⡯⡏⡏⡇⠃⡀⡤⡤⡄⠁⡯⡯⡯⡇⡇⠀⠀⡄⡄⠀⠀⡄⡄⡆⡁⠀⡴⡯⡿⡯⡿⠏⠿⡯
⡀⠀⠀⠀⠀⠀⠉⠏⡏⡇⠯⡯⡯⡏⠏⠠⣾⣿⣿⡃⡄⠀⠫⡯⡯⡯⡯⠀⠁⠼⣧⠀⡇⠅⡯⡍⠅⠯⡿⡯⡿⡯⡧⡭⡯⡯
⡿⡷⡦⡀⠀⠀⠂⠃⡏⠇⡇⠏⡏⠏⡁⣿⣿⣿⣿⠍⠁⠀⠈⡏⠏⠏⡏⠀⠄⡧⡏⡤⡇⠁⣯⡯⡯⡧⡍⠋⡿⡿⡏⠭⡭⠍
⡿⡿⡿⡿⡿⡷⠃⡮⣯⣷⡇⡇⡇⠇⡿⡮⡛⡿⡿⡷⡥⡥⡆⡇⠏⠇⡏⠅⡏⡯⡯⡧⡿⡷⡏⡿⣿⡿⠇⡼⡿⡿⡿⡧⡯⡿
⡿⡿⡿⡿⡟⡡⡯⡯⣯⡿⡧⢧⣧⡿⡟⡏⡯⡏⡯⡯⡯⡯⣿⡧⡏⡇⡯⡏⠏⠇⠏⡿⣿⣿⡏⡿⠏⡥⡿⡿⡿⡿⡿⡿⡿⡿
⡿⡿⡿⡏⡂⠯⣿⣿⣿⡿⡿⡏⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠁⠁⠁⢨⣿⣿⡏⡷⠇⡩⡿⡿⡿⡿⡿⡿⡿⡿
⡿⡿⡿⡿⡿⡷⡦⠍⡉⡉⣿⡯⡏⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿⣦⡄⡀⣤⣿⡿⠏⡋⡦⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿
⡿⡿⡿⡿⡿⡿⡿⡧⡭⡍⡋⡋⠯⠏⠏⠏⠛⠿⣿⣿⣿⣿⡿⠋⠋⠛⠟⠛⠿⣿⡿⠏⡥⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿
⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⠇⡆⠀⡄⠉⠋⠛⠿⠶⣦⣦⣤⡤⠄⠋⠥⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿
⡿⡿⡿⡿⡿⡿⡿⠏⠋⠋⠟⠟⠟⠟⠟⠨⡿⡇⡯⡯⡧⡆⡆⡆⠆⠏⡍⠆⡦⡦⡇⠏⠿⠿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿
⡿⡿⡿⡿⡿⠏⣥⡦⡅⡀⡄⡀⠁⠀⠀⠅⠍⠉⠟⡏⠿⡧⡇⠇⡇⠅⠇⡇⠿⠭⠇⣸⣷⣷⣷⣦⣯⣭⡛⠯⡿⡿⡿⡿⡿⡿
⡿⡿⡿⡯⢋⣾⣿⣿⣿⣿⡿⡯⡧⡧⡷⡦⡄⠀⠀⠃⡍⠏⡃⠇⠇⠥⠅⡁⠋⡃⣧⣿⣿⣿⣿⣿⣿⣿⡇⣷⡌⠯⡿⡯⡿⡯
⡿⡯⡿⠁⣿⣿⣿⣿⣿⣿⣿⣿⣿⣯⡿⡯⡿⡆⠀⠀⣿⣯⣯⣟⣟⡇⡯⡿⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⣿⣿⡇⠿⡯⡿⡯
⡿⡯⡿⠁⠿⡿⡿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡧⡄⠀⣿⣿⣿⣿⣿⣧⣍⣍⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⣿⣿⣿⣇⠫⡿⡯
    Mao has crashed uwu (vewry bad)"#,
  r#":: CRASH SPLASH 1 ::
⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠋⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠉⠋⠛⠛⠛⠟⠿⠿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⡿⠏⠁⣤⣶⣿⣿⣿⡿⠿⠛⣯⣭⡷⡷⠷⠷⠷⠷⠷⡧⣯⣭⣭⣭⣭⣥⣥⣤⣤⡤⡤⡄⡉⠉⠻⠿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⡿⠃⣠⣿⣿⣿⡿⠟⣯⠷⣛⡯⠭⠗⡖⡓⣛⡛⡛⣿⣷⣷⣷⣧⣯⣯⣯⡯⡧⠧⠷⠶⣷⣦⣿⣷⣦⡄⠉⠻⣿⣿⣿⣿
⣿⣿⣿⣿⣿⡟⠁⣰⣿⣿⣿⣯⡿⢏⡥⣋⣥⣶⣿⣿⣿⣿⣿⣿⣷⡍⣿⣿⣿⣿⣿⣿⡷⢶⣷⣿⣿⣷⣦⣍⠿⣿⣿⣿⣧⠀⠹⣿⣿⣿
⣿⣿⣿⣿⡟⠁⣠⣿⣿⣿⣿⣿⣷⣯⣵⠿⠟⠋⠋⠉⠉⠉⠋⠛⠿⣷⣿⣿⣿⣿⣿⣿⡇⣿⣿⡿⡿⡿⡿⣿⣧⣿⣿⣿⣿⡇⠈⢿⣿⣿
⣿⣿⡟⠁⠀⠈⠟⠿⡿⣿⣿⠟⠛⡿⠃⠀⠂⠀⠀⠀⠀⠚⠿⡷⣄⠈⠻⣿⣿⣿⡿⡿⠟⠋⠁⠀⠀⠀⠀⠀⠹⡟⠿⠿⠿⠿⣦⡀⠙⢿
⡿⠃⡠⣢⣿⠟⠋⠉⡉⡉⠉⠛⠧⡿⣷⣶⣶⡿⠋⠁⣷⣶⣦⣄⡀⣀⣴⣿⣿⣿⣧⡄⠀⣠⣤⣶⣶⣶⣷⣿⣿⡿⡿⡿⣿⡛⠦⡝⠆⠈
⠁⣤⡇⣿⠏⢀⣶⣿⡏⠉⠷⣦⣤⣄⣄⣉⣁⣤⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠻⣿⣿⣿⡏⠋⠛⠋⠁⣤⣤⣄⡉⡇⣿⡎⠀
⠀⣿⡏⣿⠀⠿⠟⠋⠁⠠⣄⡉⠛⠛⠿⣿⣿⣿⠿⠟⠟⡟⡻⠋⠉⣉⣿⣿⣿⣿⣿⣿⣧⣄⠈⠛⠿⣿⣷⣶⣶⣿⠃⠻⣿⡿⠏⡿⠁⠀
⡀⠙⡇⠿⡇⠈⣿⣷⡆⠈⠿⣿⣷⣦⠀⠀⠉⠛⠛⠿⠿⣿⣇⠀⠻⡏⣁⡁⣉⣿⡿⣿⡿⠏⠁⣄⣤⣍⡛⠿⡿⠏⠀⠀⢻⠷⣟⡥⠁⣠
⣿⡄⠉⠧⠭⣛⣿⣿⣿⡄⠀⠀⠉⠛⠇⠉⠿⣷⣶⣦⣄⡀⠉⠉⠛⠛⠿⠿⠿⠿⡧⡀⣀⡤⡿⡿⠿⠿⠟⠋⠁⠀⠀⠀⠸⣿⣿⠁⣠⣿
⣿⣿⣧⡄⠀⢿⣿⣿⣿⣿⣧⠀⠻⣦⡄⠀⠀⠉⠉⠛⠿⠇⢠⣿⣿⣷⣦⡄⠀⢠⣄⣄⡀⠀⣀⣤⣤⡄⠀⣿⡇⠀⠇⠀⠈⣿⡯⠀⣿⣿
⣿⣿⣿⣿⡇⠈⢿⣿⣿⣿⣿⣷⡄⠉⠃⣸⣿⣶⣦⣄⡀⠀⠀⠉⠉⠉⠛⠋⠀⠉⠛⠋⠃⠀⠋⠋⠋⠁⠀⠉⠁⠀⠀⠀⠨⣿⡯⠀⣿⣿
⣿⣿⣿⣿⣿⣆⠀⠻⣿⣿⣿⣿⣿⣧⣄⠉⠛⠿⣿⣿⠇⠀⣷⣦⣦⣤⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⡯⠀⣿⣿
⣿⣿⣿⣿⣿⣿⣷⡄⠉⠿⣿⠿⣿⡿⠿⣷⣦⡄⠉⠋⠀⠾⣿⣿⣿⣿⣿⠀⣿⣿⣿⣷⠀⣠⣶⡆⠀⣴⠆⠀⣎⡀⠀⣠⣿⣿⡯⠀⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣷⡄⠉⠓⠮⣍⡳⠦⣍⡛⠿⣧⣤⣄⡀⡉⠉⠋⠋⠀⠛⠟⠿⠏⠀⠟⠟⠁⠀⠋⠀⠈⣁⣠⣶⣿⣿⣿⡿⠀⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣄⡀⠙⠷⠦⣍⡛⠧⣭⣍⡻⠿⣿⣿⣿⣿⣷⡷⡷⡷⡷⡷⣷⣿⣿⣿⣿⣿⡿⠟⣿⣿⡿⣿⣿⠀⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣄⡀⠉⠛⠷⣧⣯⣅⣗⡃⡯⠭⠭⡭⣭⣍⣍⣏⣏⣏⣏⣏⣭⣥⣷⣿⡿⠋⣥⣿⣿⠀⢿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣤⡀⠉⠛⠿⡿⣿⣿⣷⣷⣷⣧⣯⣯⣯⣯⣯⣯⣯⣯⣯⣧⣾⣿⣿⡿⠏⠀⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⣤⣄⣁⡉⠉⠛⠛⠟⠿⠿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠟⠁⣠⣾⣿⣿
   The code just trolled us (it's broken)"#,
   r#":: CRASH SPLASH 2 ::
   ⠏⠿⡍⡇⠁⠁⠁⣥⡍⠧⡿⠏⠋⡏⠁⠁⠁⡭⠿⠧⡉⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠀⠁⠉⠄⠉⠁⠧⡭⣧⡧⠅⡇⣿⡇⠀⠀⡽⠇⠨⡿⠻⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠀⠀⠀⠀⠆⠈⠃⡿⣿⡯⠇⡇⣿⡿⠄⡠⠇⠁⠬⡇⠀⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠅⠁⠀⠀⠀⠄⠁⠄⠛⡿⡏⠁⠇⡿⡏⡧⠏⠁⠀⠯⠧⠇⠹⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠏⠇⠁⠁⠀⠄⡇⡄⠃⡄⠁⠸⠁⡿⡧⠏⠁⠀⡀⡘⡏⡇⠄⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⠏⡇⠉⠟⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠅⠁⠅⠃⠇⡏⡯⡇⡦⠁⡇⠠⡏⡇⠀⠀⡿⡏⠉⠹⡏⡇⠿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⣿⡿⡏⠇⡇⡇⡇⡍⠯⡯⡯⡯⡯⡏⡏⡏⡏⡏⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠇⠅⠅⠅⠇⡇⡏⡏⡏⠀⠁⡏⠀⠀⡠⡿⡿⡧⠀⠀⠙⡍⡧⡍⠿⡿⡿⡿⡿⡿⡿⡯⡿⡯⡯⡯⡯⡏⡏⡯⣿⡿⣿⣿⣿⡏⡇⡇⡏⡇⡏⠇⡇⡏⡯⡏⡏⠏⡏⡏⡇⡏⡏⡏⡏⡏⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠇⠅⠅⠅⡇⡇⡯⡧⠇⠀⠠⠇⠀⠀⠉⠯⡿⡿⡧⠀⠀⠉⡏⠧⡕⡍⠿⡯⡿⡯⡿⡯⡿⡯⡏⡏⡏⡇⡯⡿⣿⣿⣿⣿⣿⡇⡇⡇⡯⡯⡯⡧⡧⡇⡏⡏⡇⡇⡧⡇⡇⡇⡧⡇⡏⡇⡏⡏⡏⡏⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡿⠏⡍⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠇⠅⠅⡅⡧⠏⠅⠃⠁⠀⡯⠀⡏⠀⠄⠈⢿⡯⡿⡧⡄⠀⠈⠇⠇⠌⡧⡯⡿⡯⡿⡏⡏⠇⠁⡇⡿⡿⣿⣿⣿⣿⣿⣿⣿⡇⡿⠏⡯⡯⡯⡯⡧⡏⠇⡯⡯⠏⠏⡏⡯⡯⡯⡯⡯⡏⡏⡇⡯⡏⡏⡏⡟⡯⣿⡿⣿⡿⡿⡟⡏⡏⡧⡧⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⡇⡧⠏⠅⠃⠁⠄⠀⠁⠀⡇⠀⡯⠅⡯⡄⠈⡿⡿⡿⡧⠄⠀⠀⠀⠁⠅⡅⡯⡏⡯⡏⡯⡏⡯⡿⣿⣿⣿⣿⣿⡿⡿⡿⡿⡧⡿⡯⡯⠏⠟⠏⠏⠏⡧⡿⡯⠇⠅⠅⠅⡇⡇⠉⡏⠏⡏⠏⡏⠏⡿⡏⠏⠏⠏⠏⡏⡇⡯⡯⡯⡏⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠧⡯⠇⠁⠀⠁⠀⡄⡆⡇⠠⡇⠀⡯⠇⡿⡇⠆⠈⡿⡿⡿⡇⠄⠀⠀⠀⠀⠧⡇⡯⡏⠇⠏⡏⣿⣿⣿⣿⣿⣿⣿⡿⡿⡿⡿⡯⡿⡏⠟⠟⠟⡇⡧⡇⡯⡏⡏⠇⠁⠇⠀⠁⠇⠄⠁⠅⠅⠅⠏⠇⠇⡇⡇⡥⡷⡯⡿⡯⡿⡯⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠏⠁⠀⠀⠄⠇⠇⠇⠇⠇⠨⠇⠁⡏⠅⡏⡏⡇⠄⠹⡯⡿⡯⡧⠀⠀⠀⠀⠀⠇⠏⡧⡧⡯⡯⣿⡿⣿⡿⣿⡿⡿⡿⡯⡯⡯⡯⡧⡧⡧⡧⡭⡏⡏⡏⡏⡇⠇⠅⡅⠅⡄⠄⠁⠇⠀⠁⠁⠅⠅⠁⠀⠇⡇⡯⡿⡯⡯⡯⡿⡯⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠅⠀⠄⠅⠅⠇⠇⠇⠇⠅⠨⠇⠀⡇⠁⡯⡯⡿⡇⠃⡿⡿⡏⡿⡇⠀⠀⠀⠀⠁⠇⡏⡇⡿⡿⣿⡿⣿⡿⣿⡿⡿⡯⡯⡯⡇⡏⡏⡏⡯⡏⡯⡏⡏⡏⡇⡇⡇⡇⡇⡇⡏⠇⠁⠇⡇⡄⠁⠁⡄⠇⡇⠅⡇⡇⡏⡏⡯⡯⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠅⠅⠁⠅⠅⠇⠅⠇⠅⠁⠨⡇⠨⠇⠁⣯⡿⣿⣿⣿⡿⣿⡿⡏⡯⡇⠀⠀⠀⠀⠅⠉⡍⣏⡏⡿⡿⡿⡿⣿⡿⣿⡿⡿⡯⡧⡏⡯⡏⡯⡏⡯⡏⡯⡯⡯⡏⡯⡯⡯⡯⡇⠃⡅⠁⡿⡇⡀⡧⡇⡯⡏⡇⠏⡇⡇⡇⡇⡏⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠁⠁⠅⠁⠅⠅⠇⠅⠀⠈⡧⣇⡷⠿⣯⡿⡿⡿⡿⡿⠟⡏⠿⡏⡇⠀⠀⠀⠀⠁⡄⠀⠋⠁⠹⡯⡿⡯⡿⡯⡿⡿⡿⡯⡯⡏⡯⡏⡯⡏⡯⡯⡿⡯⡿⡿⡿⡯⡿⡯⠇⠀⡯⠄⠏⠏⡿⣿⡧⡿⡿⡇⡏⡧⡯⡏⡧⡇⡯⡯⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠁⡀⠅⡇⡇⠏⠇⠀⠀⡄⡿⡯⣿⡿⡿⡯⡯⡏⠧⡉⠣⡀⠄⠿⡿⠅⠀⠀⠀⠀⠏⠄⡂⡍⡍⡏⡯⡏⡯⡯⡿⡿⡿⠏⡏⡏⡯⡯⡏⠏⡏⡏⡏⡯⡿⡯⡿⡯⡿⡯⠅⡯⡿⡇⠅⡏⡿⡿⣿⡿⡿⡿⡏⡯⡿⡯⡯⡏⡏⡏⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⡿⣿⡧⡅⠁⠁⠀⡤⡯⠏⡇⠉⡏⠏⠍⣿⡷⡿⡿⣧⡍⠧⡉⠆⠀⠿⠇⠀⠀⠀⠁⠀⡆⡿⡷⣯⡏⣯⡏⡯⡏⡿⡿⡿⡇⡏⠇⠃⠁⡇⡇⡏⡏⡏⠏⠟⠏⡏⠋⠟⠏⠇⠏⡏⡇⠁⠇⡿⡿⡿⡿⡿⡯⠏⡯⡯⡯⡿⡯⡯⡇⡇⡏⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⡿⠿⡿⣿⡦⣤⡯⡟⡇⡏⠇⠀⠁⡇⠍⡯⡧⣿⡿⣿⡿⣧⡅⡌⠄⠀⡅⠀⠀⠁⠀⠁⡅⠏⡿⡿⡿⡿⡏⡏⠏⠏⠅⠏⡅⡁⠇⡇⡯⡿⡯⡏⡏⡇⠧⡅⡇⡯⡏⡧⡦⡄⡈⣿⡯⣧⡧⣿⡿⡿⡯⡯⠇⠃⠏⠏⠏⠏⡋⠋⠏⡏⡏⡯⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠟⡿⡏⣿⡿⣿⡯⡇⡯⠇⠁⠀⠀⠀⠃⡉⠋⠿⠿⣿⡿⡿⡿⡇⡄⠀⠉⡀⠁⡄⠀⠀⠁⡇⡿⡿⡿⡿⡏⡇⡇⠇⠯⡏⠍⠷⠇⡟⡏⠏⡏⡿⡿⡿⠷⣯⡿⣿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡇⡶⡷⡇⡅⠁⠁⡋⡯⡏⠇⠇⡏⡯⡿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⠉⠛⡏⡭⡿⣿⡏⠇⡏⠁⠀⠀⠁⠠⣦⡁⠄⡁⠋⠏⠿⡿⡿⡇⡄⠀⠇⠀⠁⠀⠀⠀⠉⠇⡿⡯⡏⠅⠅⠅⠅⠁⠃⠇⠆⠀⠀⠁⠉⠇⡧⡯⡿⡿⡿⡿⣿⡿⣿⡿⣿⣿⣿⡿⣿⣿⣿⡿⣿⡿⣿⣿⣿⡿⡿⡯⡍⡥⡏⡇⠀⠁⠏⠏⡿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠅⠇⠇⡍⠏⠧⠏⡇⠏⡇⠀⠀⠀⠀⠩⣿⡯⣧⣦⣯⡧⡆⡄⠉⠇⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠏⡧⡇⡧⡇⡇⠅⠄⠄⠄⠁⠁⠁⠃⡃⠁⡍⡯⡿⡿⡿⣿⡿⣿⡿⣿⣿⣿⡿⣿⡿⡿⡿⣿⡿⣿⡿⣿⡿⡿⡯⡏⠏⠏⠇⡇⡇⡇⡧⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠅⠀⡇⠁⠅⡇⡏⠇⠋⡇⠀⠀⠀⠀⠩⡿⡏⣯⡭⡭⡏⠿⡯⡏⠆⡍⠆⡀⠁⠿⡦⡦⡄⠀⠀⠀⠀⠉⠋⡏⠇⡇⠇⠇⠁⡁⠅⠇⠉⡁⠁⡏⡏⡏⡇⠋⠏⡿⡿⡿⡿⡿⡿⣿⡿⡏⠋⠿⡯⡏⠯⣿⡿⣿⡿⡿⡏⠇⠁⠀⠁⠏⡏⡏⡿⡿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠅⠀⠀⡁⡄⡏⡏⠇⡅⡇⡀⠀⠀⠀⠹⡯⡧⡶⠆⣂⡄⠀⡄⠁⠅⠄⠁⠉⠃⡿⡯⡿⡯⡏⡇⡇⠀⠀⠀⠀⠁⠃⠅⠅⠁⠅⠄⠋⠁⠀⠈⡏⡏⡯⠃⠃⠃⠀⠋⡿⡿⡿⡿⡿⡿⡿⡇⠃⠁⡧⡿⡿⡿⡿⡿⡟⡏⡇⡆⡄⡄⡯⡯⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠇⡀⠀⡃⠅⡅⡇⡯⠉⡇⠇⠀⠀⠀⠩⡿⠋⡅⡥⠟⠇⠁⠁⡬⡧⡇⠀⠁⠡⡥⡭⡯⡏⡏⡏⡇⡀⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠇⡏⡯⡇⠄⡅⠇⡍⡇⡇⠄⠉⠏⠿⠏⠏⠃⡿⡿⡿⡯⡿⡿⡿⡿⡯⡏⡯⡯⡿⡯⡿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠯⣿⣿⡿⠇⡋⡯⡏⠏⡇⠀⠀⠀⠀⠨⡇⡄⠁⡇⡅⠀⠁⡯⡿⡯⡇⠀⠀⠆⠫⡏⡏⡯⡿⡿⡿⠏⠃⠀⡁⠂⠄⠀⠀⠀⠁⠀⠀⠀⡎⡇⡯⡏⡋⡇⡇⠁⠏⠯⡿⡇⡅⡄⡄⡄⡄⡄⡍⡏⡿⡯⡿⡏⠏⠏⠏⠏⡿⡯⡿⡯⡿⡯⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠿⣿⣿⡏⠅⠏⠏⠏⠁⠁⠀⠀⠀⠀⠨⠇⠁⠏⡏⠁⡀⡏⡏⡏⡏⠃⠀⠀⠅⠀⠉⠉⠉⠁⠀⠀⡁⡧⡥⡶⡯⡧⡅⡄⠀⠀⠀⠀⠀⠇⡇⡯⠇⡯⡏⠃⠀⠀⠀⠉⠋⠿⡿⡯⡯⡯⡯⡿⡯⠏⠁⠁⠁⠁⠀⠅⠅⡏⡏⡿⡿⣿⡯⡯⡯⣿⡿⣿⡿⣿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⣿⣿⡿⡇⠏⠃⠁⠀⡦⡦⡶⡿⡯⡿⠿⠅⠇⠇⠅⠦⠏⠿⠏⠿⠇⠀⠀⡀⠩⡧⡧⡇⡇⡇⡅⡏⡏⡍⠏⠏⠏⡿⡏⡏⠇⠇⠄⠀⡇⡯⡯⡏⡇⡷⡇⠀⠀⠀⠀⠀⠀⠄⠁⠍⠋⡏⡏⡇⠀⠁⠀⠀⠀⠁⠁⡇⡇⡏⡯⡿⡯⡿⡿⡿⡯⣿⡿⡿⡯⡿⡿⡿⡿⡇⠀⠀⠀
⠀⠀⠀⡿⡿⡿⠇⠀⠀⠀⠀⠁⠁⠁⠁⠁⠁⠈⡇⠀⠀⠀⠀⠀⠀⠀⠀⠄⠄⠄⠆⡏⡏⡍⡍⡏⠯⡏⠏⠋⠍⠃⠇⡆⡄⡄⡄⠀⠀⠀⠯⠏⡿⡯⠯⡏⡏⠁⠀⠀⠅⠅⠀⠀⠇⠇⠇⠄⠁⠇⡇⠁⠀⠀⠀⠀⠀⠀⠏⠃⡏⠏⡏⡯⡿⡿⡿⡯⡿⡯⣯⡿⣯⡯⡿⡿⡇⠀⠀⠀
⠀⠀⠀⡯⡯⡏⠁⠀⠤⠆⠄⡅⠃⠧⡇⡄⡇⠀⡯⡇⡇⡧⡇⠁⠇⠄⡀⠀⠀⠉⠍⠃⠅⠍⡇⡃⡋⠏⠇⠀⠀⠃⠀⠄⡭⠯⠏⠁⠇⠇⡄⠀⠀⠄⠀⠀⠁⠀⠀⠀⠁⠁⠀⠀⠇⠁⠁⠀⠁⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠁⠉⠋⡏⡯⡏⡿⡯⣿⡯⡿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⠿⡿⡧⡄⡄⡅⡄⡏⡏⠯⡯⣿⡿⣿⣷⡯⡏⡍⡍⡃⠇⡇⡇⡅⡏⡇⠂⠇⠇⠃⡏⡏⡇⠏⠃⡏⡏⠏⠇⡏⡇⡧⡦⡥⡥⡿⡏⡧⡥⡧⡦⡆⠇⡇⡦⡦⡦⡄⡆⡆⡄⡄⡄⡄⡀⡀⡄⡄⡄⠄⠅⡇⡦⡏⡏⡏⡏⠏⡋⠿⡯⡿⡯⣯⡯⡟⡿⣿⡿⡿⡿⣿⡿⡇⠀⠀⠀
⠀⠀⠀⡯⡯⡟⡿⡿⡿⡏⡟⡟⡧⡿⡿⣿⣿⡿⡿⠿⡧⡏⡯⡯⡿⡿⡿⡯⣿⡷⡧⡷⡷⡯⡦⡮⡿⡷⡷⡿⣿⡷⡧⡯⡿⡿⡿⡿⣿⡯⣷⡿⡿⡿⡯⡷⣿⡿⡯⡯⡏⡏⡭⡇⡿⡿⡿⠏⡟⡿⡧⡍⡿⡿⡿⡷⣯⡥⡥⡏⡗⡗⡏⠏⡿⡿⣿⡯⡿⡿⡿⡿⡏⡏⡯⡿⡇⠀⠀⠀
⠀⠀⠀⠉⠋⠋⠋⠃⠃⠋⠋⠃⠋⠋⠋⠋⠋⠋⠃⠂⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠃⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠃⠋⠃⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠃⠃⠃⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠋⠃⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⡟⠟⠗⠀⣶⡟⢿⣦⠀⣶⡟⢿⣦⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠿⠗⢻⣷⠀⣿⡇⢸⣿⠀⣿⡇⢽⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣦⡆⢸⣿⠁⣿⡇⢸⣿⠀⣿⡇⢽⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠙⠿⠿⠟⠀⠙⠷⠿⠋⠀⠛⠷⠿⠋⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
                                       Internal Server Errror

   Well, not really but the code *is* broken."#,
   r#":: CRASH SPLASH 3 ::
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡀⡀⣄⣥⣤⣤⣷⣶⣶⣶⣶⣦⣤⣤⣄⣅⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⣄⣦⣿⣿⣿⣿⣿⡿⣿⡿⣿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠤⠇⣿⡿⠿⡿⡿⠏⡏⠏⡏⡏⡏⡏⡿⡏⡿⡏⡿⡿⣿⣿⣿⣿⣿⣷⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡅⡇⠏⠅⠅⠇⠇⡇⡇⡏⡇⡏⡇⡇⡇⡏⠇⡏⠇⠏⠏⣿⣿⣿⣿⣿⣿⣿⡧⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠇⠇⠅⠅⠅⡇⡇⡏⡇⡏⡇⡇⡇⡇⡇⡇⠇⡇⠇⠇⠅⠏⡿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠅⠅⠅⠁⠇⠇⡏⡇⡏⡏⡏⡇⡏⡇⡏⡇⡇⡇⡇⠇⠇⠅⠁⠏⢿⣿⣿⣿⣿⣿⣯⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠁⠁⠇⠇⡇⡇⡏⡇⡏⡏⡯⡧⡿⡏⡯⡏⡇⠇⠇⠁⠁⠁⠁⠟⠏⠏⣿⣿⣿⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠁⠇⠇⡇⠇⡇⡇⡏⡏⡏⡏⡏⡏⡏⠇⡇⠇⠇⠁⠁⠀⠁⠁⠇⡅⡿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠅⠅⠇⠏⡏⡇⡇⡇⡏⡇⡏⡏⡇⡇⡇⠇⠅⠁⠁⠀⠁⡧⣿⡯⣿⣿⣿⡇⠅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠁⠁⡅⠇⡇⠇⡇⡇⠏⠏⠏⠇⡏⠏⠏⠇⠇⠁⠅⠅⠁⠁⡇⡻⣿⣿⣿⣿⣿⣿⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠅⠅⠁⠁⠇⠇⠇⠇⠇⠁⠁⠁⠁⠁⠇⠁⠁⠁⠁⠁⠁⠁⠃⠉⠇⠀⠏⡿⡿⣿⣿⡟⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠅⠅⠅⠅⡇⡅⡅⡇⡧⣧⣧⡧⡧⡇⡇⡇⡅⡄⡇⡇⡧⡧⡧⡇⡏⡅⡇⡏⡅⠟⡿⡇⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠁⠁⠁⠇⠅⡇⡇⡏⡋⡏⡏⡇⠅⡇⠅⠁⠁⠅⠇⡏⠏⡏⠅⠅⠁⠫⡇⡇⣧⠁⠁⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⡇⠀⠀⠁⠀⠅⡇⡏⡇⡯⡏⡇⠏⠇⠇⡇⡇⠅⠁⠁⠇⠏⠏⡇⠇⠇⠧⣷⡍⣿⡟⠄⠅⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠧⡇⠅⠀⠁⠇⠏⠏⠇⠇⠁⠁⠁⡇⠅⡇⡇⠅⠀⠀⠀⠁⠁⠅⠅⠅⠁⠉⠙⡏⡇⠥⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⡇⠄⠀⠁⠁⠅⠅⠁⠁⠀⠄⠀⠁⠁⠁⠁⠁⠀⠀⠀⠁⠀⠀⠁⠁⠀⠀⠀⡇⠇⠅⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⡇⠁⠀⠀⠁⠁⠀⠀⠀⠅⠇⠇⠄⠀⠀⠀⠀⠁⠁⠁⠀⠀⠀⠀⠀⠀⠀⠇⠇⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡅⡅⡁⡀⡄⠀⠄⠀⠀⠀⠄⠀⠀⠀⠀⠀⡍⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣤⣄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠅⠇⠅⠇⠄⠉⠋⠋⠃⠋⠁⠁⠀⠁⠁⠅⠅⠇⠁⠀⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣿⣿⣧⣴⣦⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠇⠇⠇⠅⠇⠅⠁⠉⠁⠁⠁⠁⠁⠁⠅⠁⠁⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠙⣿⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠇⠇⠇⡅⠅⠁⠀⠁⠁⠄⠄⠅⠁⠁⠀⠀⠀⠀⡅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢀⣍⡟⠏⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠇⠇⠇⠇⠇⠇⠅⠅⠁⠁⠀⠀⠀⠀⠀⠁⣧⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠹⡿⠇⠀⢻⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠁⠁⠁⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⣀⣾⣿⣇⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⡤⣧⡅⠀⠈⠏⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡅⡇⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡴⣿⣿⣿⣿⡌⣧⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠿⣿⡿⠇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣇⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⣿⡿⠅⠹⡿⣷⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠁⡿⡇⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣷⣇⡀⠀⠀⠀⠀⠀⠀⠀⣡⣶⣿⣿⣿⣿⣿⡿⡇⡇⠀⠁⠏⠿⡿⠇⠗⠆⠄⡀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣠⣿⣯⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⠷⠅⠅⠆⠁⠀⠛⠿⣿⣿⣿⣿⣿⡿⡇⣿⠇⠀⠀⠁⠁⠏⠇⠀⠁⠀⠁⠉⠃⠦⠄⡄⡀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠙⣿⡟⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⣿⣿⣿⡿⠏⠁⠀⠃⡀⠄⠀⠀⠀⠀⣀⡙⢿⣿⣿⣿⡏⡿⠀⠀⠀⠀⠀⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⠁⠉⠋⠗⠦⡄⡀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠨⣿⡿⠏⣡⣾⣷⡄⠀⠏⠁⠄⠀⢀⣾⣿⣿⣦⣝⡿⣿⣯⡏⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠉⠋⠗
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠨⣿⣥⣿⣿⣿⡿⠏⠀⠇⡇⠀⠁⠿⣿⡿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
Dark Brandon rises, crashing the code!"#,

   r#":: CRASH SPLASH 4 ::
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠄⠄⠄⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠅⠁⠅⠅⠇⠅⠇⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠅⠇⡇⡇⡇⡇⠅⠇⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⡇⡇⡏⡏⡏⡇⡇⠅⠇⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠇⠇⡏⡏⡏⡇⡏⡇⡇⠅⠇⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡅⡇⡯⡏⡯⡏⡯⡇⡏⠇⠅⠇⡅⠁⠅⠁⠁⠁⠁⠁⠁⠁⠁⠀⠁⠁⠁⠁⠆⠄⠄⡄⡄⠇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡯⡯⡿⡯⡯⡏⡯⡏⡏⡇⡇⠇⠇⠁⠁⠀⠀⠀⠁⠀⠀⠀⠁⠀⠀⠀⠁⠀⠁⠀⠅⠇⠇⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠏⡏⡿⡯⡯⡯⡯⡏⡏⡇⠇⠁⠁⠀⠀⠀⠅⠅⠅⠀⠁⠀⠁⠀⠀⠀⠅⠁⡅⠇⡇⠅⡅⡅
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡯⡏⡏⡏⡏⡏⡏⠇⠇⠅⠅⠅⡅⡄⡥⡧⣯⡧⡇⡅⠅⠀⠀⠀⠁⠁⠁⠅⠇⠅⠇⡏⡏⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠠⡿⡏⡏⡇⡇⡇⡇⠇⡇⡇⡇⡯⡧⣿⣿⣿⣿⣿⣿⡏⡇⠀⠇⠇⡇⠅⠅⠁⠅⠁⠅⠅⠇⠇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠭⡿⡏⡏⡇⡇⡇⡇⡇⡏⡇⣿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠇⡇⡇⠇⠇⠇⠅⠅⠇⠇⠇⠅
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠌⡏⡏⡇⡇⡧⡇⡯⡯⡯⡿⡿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⡯⡧⡇⠇⡇⡅⠇⠅⠇⠅⠇⠇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠅⠇⠇⠇⠏⠏⠏⠏⠏⡯⡿⡿⡿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⡏⡇⡇⡇⡇⡇⡇⠇⡇⠇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠅⠇⡇⡇⠁⠁⠀⠀⠀⠇⠁⠏⡯⡿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡿⡯⡿⡏⡯⡯⡯⡇⡇⡇⠇⠇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠇⠇⠅⠇⡏⡇⠁⠀⠀⠀⠇⠀⠅⠇⡏⡯⡿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡿⡏⠏⠏⠏⠇⠅⠅⠉⠏⠇⠇⠇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣄⣤⡄⠀⠀⠀⠀⠀⠄⠇⠅⠅⠅⠇⠇⡇⡇⡄⠀⠀⠀⠅⠇⠇⡇⡿⡯⡿⡿⣿⣿⣿⣿⣿⣿⡿⡏⠇⠁⠁⠀⡅⠁⠁⠁⠅⠅⡅⠁
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣧⡄⠀⠀⠀⠀⡇⠇⠅⠅⡇⡇⡇⡯⡯⣧⣧⣧⣧⣧⡇⡧⡯⡿⡿⡿⡿⣿⣿⣿⣿⣿⡿⡏⠇⠇⠅⠅⠉⠃⠀⠁⠀⠅⡇⡇⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠨⣿⣿⣿⣿⡇⠀⠀⠀⡏⡇⠇⠅⠅⡏⡿⡯⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡿⡯⡿⡿⡿⡿⡿⡿⣿⡿⡯⡇⡇⡅⡇⠗⠇⠅⡅⡇⡇⡇⡇⡇
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣦⣶⣿⣿⣿⣿⣿⡇⠀⠀⠀⡏⡇⠅⠅⡍⡏⡯⡿⣿⡿⣿⣿⣿⣿⣿⣿⣿⡿⡯⡯⡿⡿⣿⡿⡿⡿⣿⣿⣿⣿⣿⣿⣿⡧⡧⡇⡏⡇⡇⡇⡯⡯
⠀⠀⠀⠀⠀⠀⠀⣤⣿⣿⣿⣿⣿⣿⡿⡿⣿⣿⡇⠀⠀⠀⡯⡇⡇⡇⡇⡯⡿⡯⡿⡿⣿⡿⣿⡿⣿⣿⣿⡏⡏⡯⣿⡿⡿⡿⡿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⡇⡇⡇⡯⡿⣿
⠀⠀⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⡿⡯⡿⡿⣿⡄⠀⠀⠉⡯⡧⡏⣿⣿⣿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡏⡯⡯⡿⡿⡿⡏⡯⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⡯⡯⡯⡯⣿⣿
⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⡿⡿⡯⡯⡏⡯⡯⡿⡇⠀⠀⠀⣿⣿⣿⣿⣿⣿⡿⡿⡯⡿⡯⡿⡿⡿⡿⡿⡯⡯⡏⡏⡏⡯⡿⡿⡿⣿⡿⣿⡿⣿⣿⣿⡿⡿⡿⡿⡿⡿⡿⣿⡿
⠀⠀⠀⠀⢽⣿⣿⣿⣿⣿⣿⣿⡿⡯⡏⡏⡯⡯⡯⡇⠀⠀⠀⣿⣿⣿⣿⣿⣿⡿⡿⡿⡿⡯⡿⡯⡿⡯⡯⡏⡯⡏⡯⡏⡯⡯⡿⡿⡿⡿⡿⡿⣿⡿⣿⡿⡿⡿⡿⡿⡿⡿⡿⡿
⠀⠀⠀⠀⠙⣿⣿⡿⣿⡿⡿⡿⡿⡏⡯⡏⡯⡏⡯⡿⣷⣆⡄⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⡿⡯⡯⡯⡯⡏⡯⡯⡯⡏⡯⡏⡯⡯⡿⡿⡿⡿⡿⡿⡿⡏⡿⡿⣿⡿⣿⣿⣿⣿
⠀⠀⠀⠀⠀⠉⣿⡿⣿⡿⡿⡯⡯⡯⡯⡯⡯⡏⡿⡯⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⡿⡯⡿⡿⣿⡯⡿⡯⡯⡯⡯⡯⡯⡯⡿⡯⡯⡯⡿⡯⡿⡿⣿⡿⣿⡿⣿⣿
⠀⠀⠀⠀⠀⠀⠉⠋⠿⣿⣿⡯⡿⡯⡯⡯⡿⡯⡿⡿⣿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡿⡯⡿⡿⣿⡿⡿⡿⡿⡿⣿⡿
⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⡿⡿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡿⡯⡯⡯⡯⡯⡿⡿⡿⡯⡿⡯⡿⡯⡿⡿⡿⡯⡿⡯⡿⡯⡿⡿⡿⡿
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⣿⣿⣿⣿⣿⡿⣿⣿⣿⡿⣿⡿⣿⡿⣿⡿⣿⣿⣿⡿⣿⣿⣿⣿⣿⣿⣿⡿⣿⡯⡿⡯⡯⡯⡯⡯⡿⡿⡿⡿⡿⡯⡏⡏⡏⡏⡯⡏⡏⡇⡏⡏⡏⡏
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⡿⣿⣿⣿⣿⣿⡿⣿⡿⣿⣿⡿⡿⡿⡯⡯⡯⡯⡯⡯⡯⡏⡏⡏⡇⡇⠇⠏⠇⠇⠇⠇⠅⠏⠇
The code tried, cut it some slack.
"#,
   r#":: CRASH SPLASH 5 ::
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⡿⡿⡿⡿⡿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠏⠇⠇⡇⠇⠇⠏⠿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠁⠇⠅⠇⠁⠁⠁⠃⠅⠅⠟⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠅⠁⠅⠁⠁⠅⡇⡇⡇⠁⠅⠁⠍⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠇⠁⠁⠁⠁⠁⡅⡇⠅⡅⠀⠁⠁⠁⠉⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠅⠁⠁⠁⠁⠀⠅⡯⡇⠅⠏⠁⠁⠁⠅⠀⠉⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠏⠁⠅⠁⠅⠁⠁⠀⠥⡧⡧⡇⡇⠁⠁⠀⠅⠁⠁⠀⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠏⠁⠁⠁⠁⠅⠁⠁⠀⠉⠋⠋⠁⠅⠀⠀⠀⠁⠁⠁⠀⠉⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠅⠁⠅⠀⠁⠁⠅⠀⠀⠀⠁⠁⠁⠀⠀⠀⠀⠁⠅⠁⠄⠟⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠏⠇⠅⠅⠅⠅⠁⠅⠀⠁⠁⠀⠀⠁⠁⠁⠀⠀⠀⠁⠀⠁⠅⠁⠁⠉⠛⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠏⠅⠅⠇⠅⠅⠁⠅⠁⠁⠁⠁⠀⠀⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠁⠀⠁⠁⠀⠉⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠅⠁⠅⠁⠅⠁⠅⠁⠁⠁⠁⠁⠁⠀⠁⠀⠁⠁⠁⠁⠁⠁⠀⠀⠀⠀⠀⠁⠁⠀⠁⠉⠟⠿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⠅⠁⠁⠁⠁⡅⡿⡏⠿⡇⡅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠁⠀⠀⠀⠁⠀⠁⠁⠁⠀⠁⠉⠿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⠁⠁⠁⠀⠅⡯⡏⠏⠅⠁⠯⡇⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠁⠁⠁⠁⠄⠅⠁⠿⡿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠁⠁⠁⠀⠉⠏⠇⠇⠃⠀⠁⠃⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⠀⠁⠁⠁⠀⡥⡥⡧⡅⡁⠉⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⠅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⠀⠁⠀⠇⠃⡇⡏⠇⡇⠷⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠁⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠁⠃⠁⠁⠀⠉⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⡅⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠅⠁⠅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡧⡅⠁⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠁⠅⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠅⠅⠁⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠅⠁⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠁⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠇⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠁⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠅⠅⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⣽⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠅⠁⠅⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠅⠁⠁⠀⠀⠀⠀⠀⠁⠀⠀⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⣬⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠅⠁⠁⠀⠀⠀⠀⠀⠁⠀⠀⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣧⣤⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠅⠁⠁⠅⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠅⠁⠅⠅⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠅⠀⠅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠅⠅⠁⠅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠁⠁⠅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠁⠁⠁⠅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠁⠁⠁⠅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠁⠁⠁⠅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠁⠁⠁⠅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠁⠁⠁⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠇⠁⠁⠁⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠁⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠇⠁⠁⠁⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠁⠁⠀⠁⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⠁⠁⠁⠁⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠁⠁⠁⠁⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠁⠁⠁⠁⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠇⠁⠀⠁⠁⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠁⠁⠀⠁⠀⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠁⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠉⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠇⠁⠁⠀⠁⠀⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠁⠀⠀⠀⠀⠁⠀⠁⠀⠀⠀⠀⠉⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠁⠁⠁⠁⠁⠀⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠹⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠃⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠁⠀⠁⠀⠁⠁⠁⠁⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠀⠀⠉⠛⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡯⡅⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠁⠁⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⠀⠁⠀⠁⠀⣥⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⡇⡅⡀⡁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠁⠀⠁⠀⠁⠀⠁⠀⠁⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠁⠀⠁⣤⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⣿⡇⠁⠁⠅⠁⠁⠀⠁⠁⠁⠀⠁⠀⠁⠀⠁⠀⠁⣧⣷⣧⣧⣦⣧⣅⣅⡅⡁⡅⣧⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣧⣧⣤⣧⣷⣷⣧⣧⡄⠀⠀⠁⠀⠁⠁⠁⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⣥⣥⣥⣧⣷⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿
   I *am* the error.
   "#,
  ].choose(&mut rand::thread_rng())
        .unwrap()
}