@group(0) @binding(0)
var tex1: texture_storage_2d<rgba8unorm, write>;
@group(1) @binding(1)
var tex2: texture_2d<f32>;
@group(1) @binding(2)
var tex2_sampler: sampler;

@compute @workgroup_size(16, 16, 1)
fn main(
  @builtin(global_invocation_id) pixel : vec3<u32>
) {
    var output: u32 = 0u;
    for(var i: i32 = 0; i < 32; i+=1) {
      let pixel_coord = UVBToCoord(i32(pixel.x), i32(pixel.y), i);
      let neighbors = count_neighbors(pixel_coord);
    
      if(neighbors == 3u || neighbors == 6u){
        output += 1u << u32(i);
      } else if (neighbors == 2u){
        output += (1u << u32(i)) * getPixel(pixel_coord);
      }
    }
    let outputVec = vec4(f32((output & 4278190080u) >> 24u)/255.0, f32((output & 16711680u) >> 16u)/255.0, f32((output & 65280u) >> 8u)/255.0, f32(output & 255u)/255.0);

    let pixel_coord = vec2(i32(pixel.x), i32(pixel.y));
    textureStore(tex1, vec2(i32(pixel.x), i32(pixel.y)), outputVec);
}

fn count_neighbors(pix_coord: vec2<i32> ) -> u32 {
  var sum = 0u;
  sum += is_black(getPixel(vec2(pix_coord.x + 1, pix_coord.y + 1)));
  sum += is_black(getPixel(vec2(pix_coord.x + 0, pix_coord.y + 1)));
  sum += is_black(getPixel(vec2(pix_coord.x - 1, pix_coord.y + 1)));
  sum += is_black(getPixel(vec2(pix_coord.x - 1, pix_coord.y + 0)));
  sum += is_black(getPixel(vec2(pix_coord.x + 1, pix_coord.y + 0)));
  sum += is_black(getPixel(vec2(pix_coord.x - 1, pix_coord.y - 1)));
  sum += is_black(getPixel(vec2(pix_coord.x + 0, pix_coord.y - 1)));
  sum += is_black(getPixel(vec2(pix_coord.x + 1, pix_coord.y - 1)));
  
  return sum;
}

// fn getPixel(pix_coord: vec2<i32>) -> vec4<f32> {
//   let WH = textureDimensions(tex2);
//   if(pix_coord.x < 0 || pix_coord.x > (i32(WH.x) - 1) || pix_coord.y < 0 || pix_coord.y > (i32(WH.y) - 1)){
//     return vec4(0.0, 0.0, 0.0, 0.0);
//   } else {
//     return textureLoad(tex2, vec2(pix_coord.x, pix_coord.y), 0);
//   }
// }

fn is_black(color: u32 ) -> u32{
  if(color == 1u){
    return 1u;
  } else {
    return 0u;
  }
}

fn getPixel(pix_coord: vec2<i32>) -> u32 {
//   let WH = textureDimensions(tex2);
  if(pix_coord.x < 0 || pix_coord.x > (11584 - 1) || pix_coord.y < 0 || pix_coord.y > (11584 - 1)){
    return 0u;
  } else {
    let UVB = coordToUVB(pix_coord.x, pix_coord.y);
    // if(UVB.z == 0){ return 1u; }
    // return u32(UVB.z);
    let texSample = textureLoad(tex2, UVB.xy, 0);
    return (((u32(4278190080.0*texSample.r) & 4278190080u) + (u32(16711680.0*texSample.g) & 16711680u) + (u32(65280.0*texSample.b) & 65280u) + u32(255.0*texSample.a)) & (1u << u32(UVB.z))) >> u32(UVB.z);// 
  }
}

fn coordToUVB(x: i32, y: i32) -> vec3<i32> {
  let temp1 = y * 11584 + x;
  var returnVal = vec3(0,0,0);
  returnVal.z = temp1 % 32;
  let temp2 = temp1/32;
  returnVal.x = temp2 % 2048;
  returnVal.y = temp2 / 2048;

  return returnVal;
}

fn UVBToCoord(x: i32, y: i32, b: i32) -> vec2<i32> {
  let temp = (y * 2048 + x) * 32 + b;

  return vec2(temp%11584, temp/11584);
}   