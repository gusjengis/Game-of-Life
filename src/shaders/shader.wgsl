// Vertex shader

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>
};

struct Dimensions {
    width: f32, time: f32,
    height: f32, temp: f32,
    xOff: f32, yOff: f32,
    scale: f32, dark: f32,
}

// struct Timestamp {
//     millis: f32, millis1: f32, millis2: f32, millis3: f32
// }

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;
@group(1) @binding(2)
var<uniform> dim: Dimensions;
// @group(2) @binding(3)
// var<uniform> time: Timestamp;
@group(2) @binding(3)
var t_diffuse2: texture_2d<f32>;//  texture_storage_2d<rgba8unorm, read_write>;//
@group(2)@binding(4)
var s_diffuse2: sampler;
@group(3) @binding(5)
var golTex: texture_2d<f32>;//  texture_storage_2d<rgba8unorm, read_write>;//
@group(3)@binding(6)
var golTexSamp: sampler;
// @group(3) @binding(5)
// var<storage, read_write> tex1: array<u32>;

@vertex
fn vs_main(
    in: VertexIn,
    @builtin(instance_index) instance: u32,
    @builtin(vertex_index) vertex: u32,
) -> VertexOutput {
    var out: VertexOutput;
    let aspect = dim.width/dim.height;
    // let t = f32(instance);
    // let c = time.millis;
    // let instances = 12499.0;
    // let spacing = 0.0004*20.0;
    // let xOff = spacing*sin((c/10000.0+1.0+t)*2.71828*sin(c/1100000.0))*t*(t/instances)/aspect;
    // let yOff = spacing*cos((c/10000.0+1.0+t)*2.71828*sin(c/1100000.0))*t*(t/instances);
    // let zoom = 2.0;
    // out.clip_position = vec4<f32>(
    //     (zoom)*(0.2*t/instances/aspect*(in.position.x)+xOff),
    //     (zoom)*(0.2*t/instances*in.position.y+yOff),
    //     in.position.z,
    //     1.0
    // );
    let WH = vec2(11584, 11584);//textureDimensions(golTex);
    let texAspect = f32(WH.x)/(f32(WH.y));

    let int_scaler = dim.scale;//floor(dim.height/f32(WH.y));
    var atemp = int_scaler*f32(WH.y)/dim.height;
    // atemp = floor(atemp*f32(WH.y)*int_scaler)/(f32(WH.y)*int_scaler);
    var x = texAspect*in.position.x/aspect*atemp;
    var y = in.position.y*atemp;

    x = floor((x*f32(WH.x))*int_scaler)/(f32(WH.x)*int_scaler) + 2.0*dim.xOff/dim.width;
    y = floor((y*f32(WH.y))*int_scaler)/(f32(WH.y)*int_scaler) - 2.0*dim.yOff/dim.height;

    // y = floor(y*f32(WH.y))/f32(WH.y); 

    out.clip_position = vec4(x, y, in.position.z, 1.0);
    out.tex_coords = in.tex_coords;
    // out.color = in.color;
    
    return out;
}



@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // textureStore(t_diffuse2, in.tex_coords, textureSample(t_diffuse2, s_diffuse2, in.tex_coords)+vec4(0.01, 0.01, 0.01, 1.0));
    // let xOff = 0.0;//(sin(dim.time/10000.0 + in.tex_coords.x));
    // let yOff = 0.0;//(cos(dim.time/10000.0 + in.tex_coords.y));
    // let moddedCoords = vec2(sin(dim.time/10000.0)*sin(in.tex_coords.x*100.0)+xOff, sin(dim.time/10000.0)*sin(in.tex_coords.y*100.0)+yOff);
    // let tex = golTex;
    // let texSamp = golTexSamp;

    let WH = vec2(11584, 11584);//textureDimensions(golTex);
    let int_scaler = dim.scale;//floor(dim.height/f32(WH.y));
    // let WH = vec2(u32(32), u32(1));

    let vlines = f32(WH.y);
    let hlines = f32(WH.x);
    // let height = (3840.0/dim.height)*0.5*dim.temp*0.18662*vlines/240.0;//1.0/((dim.height/256.0) - 1.0);
    // let width = (3840.0/dim.height)*0.5*dim.temp*0.18662*(320.0/240.0)*vlines/320.0;//1.0/((dim.height/256.0) - 1.0);
    
    // let pixels = floor(dim.temp);
    let pixel_coord = vec2(i32(in.tex_coords.x*f32(WH.x)), i32(in.tex_coords.y*f32(WH.y)));
    // let tex2Color = textureLoad(golTex, pixel_coord, 0);//textureSample(t_diffuse2, s_diffuse2, vec2(in.tex_coords.x, (in.tex_coords.y*240.0)%1.0));//moddedCoords);//pixelate(in.tex_coords, pixels*2.0));//512.0));
    // var color = textureSample(golTex, golTexSamp, in.tex_coords);//pixelate(tex, in.tex_coords, pixels));// + tex2Color;//256.0));
    let alive = getPixel(pixel_coord);
    // var pixColor = textureLoad(golTex, pixel_coord, 0);//textureSample(golTex, golTexSamp, in.tex_coords);//textureSample(golTex, golTexSamp, pixelate(golTex, in.tex_coords, pixels));// + tex2Color;//256.0));
    var pixColor = vec4(1.0, 1.0, 1.0 ,1.0);
    // if(alive == 1u){
    //     pixColor = vec4(0.0,0.0,0.0,1.0);
    // } else if(alive == 2u){
    //     pixColor = vec4(1.0,0.0,0.0,1.0);
    // }
    // var vPixColor = textureSample(golTex, golTexSamp, vec2(in.tex_coords.x, pixelateVertically(golTex, in.tex_coords, pixels)));// + tex2Color;//256.0));
    // if((vlines*in.tex_coords.y)%1.0 > height){
    //     discard;
    // }
    // if((hlines*in.tex_coords.x)%1.0 > width){
    //     discard;
    // }
    if((floor(vlines*int_scaler*in.tex_coords.y))%int_scaler >= int_scaler - dim.temp){
        discard;
    }
    if((floor(hlines*int_scaler*in.tex_coords.x))%int_scaler >= int_scaler - dim.temp && dim.time == 0.0){
        discard;
    }
    // if(tex2Color.r == 0.0 && tex2Color.g==0.0 && tex2Color.b == 0.0){
    //     discard;
    // }
    // if(pixColor.r == 0.0){
    //     pixColor = vec4(1.0, 1.0, 1.0, 1.0);
    // } else {
    //     pixColor = vec4(0.0, 0.0, 0.0, 1.0);
    // }
    // var avg = vec4((vPixColor.r + 2.0*pixColor.r)/3.0, (vPixColor.g + 2.0*pixColor.g)/3.0, (vPixColor.b + 2.0*pixColor.b)/3.0, 1.0);
    // if((lines*in.tex_coords.y)%1.0 > height){
        //pixColor = vec4(avg.rgb*0.01, avg.a);
    // }

    

    var out = pixColor * f32(alive);

    
    if(dim.dark == 1.0){
        out.r = 1.0 - out.r;
        out.g = 1.0 - out.g;
        out.b = 1.0 - out.b;
    }
    //SRGB Mapping
        out.r = pow(out.r, (2.2));
        out.g = pow(out.g, (2.2));
        out.b = pow(out.b, (2.2));  
    
    //Invert Colors vvv
    
    return vec4(out.r, out.g, out.b, 1.0);
}



fn pixelate(texture: texture_2d<f32>, texCoord: vec2<f32> , pixels: f32) -> vec2<f32> {
    let WH = textureDimensions(texture);
    // let WH = vec2(64u, 64u);
    var x: f32 = (floor(texCoord.x*f32(WH.x))+0.5);
    var y: f32 = (floor(texCoord.y*f32(WH.y))+0.5);
    if(x > f32(WH.x)){ x -= 1.0; }
    if(y > f32(WH.y)){ y -= 1.0; }
    x /= f32(WH.x);
    y /= f32(WH.y);
    return vec2(x, y);
}

fn pixelateVertically(texture: texture_2d<f32>, texCoord: vec2<f32> , pixels: f32) -> f32 {
    let WH = textureDimensions(texture);
    var y: f32 = (floor(texCoord.y*f32(WH.y))+0.5);
    if(y > f32(WH.y)){ y -= 1.0; }
    y /= f32(WH.y);
    return y;
}

fn getPixel(pix_coord: vec2<i32>) -> u32 {
//   let WH = textureDimensions(tex2);
  if(pix_coord.x < 0 || pix_coord.x > (11584 - 1) || pix_coord.y < 0 || pix_coord.y > (11584 - 1)){
    return 0u;
  } else {
    let UVB = coordToUVB(pix_coord.x, pix_coord.y);
    // if(UVB.z == 0){ return 1u; }
    // return u32(UVB.z);
    let texSample = textureLoad(golTex, UVB.xy, 0);
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