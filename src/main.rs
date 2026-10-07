use std::fs::{create_dir_all, File};
use std::io::{Result, Write};
use std::path::Path;

const W: f64 = 900.0;
const H: f64 = 540.0;
const L: f64 = 105.0;
const R: f64 = 35.0;
const T: f64 = 55.0;
const BTM: f64 = 85.0;

struct Svg { s: String }
impl Svg {
    fn new(title: &str) -> Self {
        let mut s=String::new();
        s.push_str(&format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}"><rect width="100%" height="100%" fill="white"/><style>text{{font-family:Arial,Helvetica,sans-serif;fill:#111}}.axis{{stroke:#111;stroke-width:1.5}}.grid{{stroke:#bbb;stroke-width:0.7;opacity:.45}}.curve{{fill:none;stroke-width:3}}.dash{{stroke-dasharray:9 6}}</style><text x="{x}" y="34" text-anchor="middle" font-size="25">{title}</text>"#, x=W/2.0));
        Self{s}
    }
    fn line(&mut self,x1:f64,y1:f64,x2:f64,y2:f64,class:&str,stroke:&str){self.s.push_str(&format!(r#"<line x1="{x1:.2}" y1="{y1:.2}" x2="{x2:.2}" y2="{y2:.2}" class="{class}" stroke="{stroke}"/>"#));}
    fn text(&mut self,x:f64,y:f64,t:&str,size:u32,anchor:&str){self.s.push_str(&format!(r#"<text x="{x:.2}" y="{y:.2}" font-size="{size}" text-anchor="{anchor}">{t}</text>"#));}
    fn poly(&mut self, pts:&[(f64,f64)], color:&str, dash:bool){let p=pts.iter().map(|(x,y)|format!("{x:.2},{y:.2}")).collect::<Vec<_>>().join(" "); let d=if dash{" dash"}else{""}; self.s.push_str(&format!(r#"<polyline points="{p}" class="curve{d}" stroke="{color}"/>"#));}
    fn circle(&mut self,x:f64,y:f64,color:&str){self.s.push_str(&format!(r#"<circle cx="{x:.2}" cy="{y:.2}" r="5.5" fill="{color}"/>"#));}
    fn finish(mut self,path:&Path)->Result<()> {self.s.push_str("</svg>\n"); let mut f=File::create(path)?; f.write_all(self.s.as_bytes())}
}
fn map(v:f64,a:f64,b:f64,c:f64,d:f64)->f64{c+(v-a)*(d-c)/(b-a)}
fn axes(svg:&mut Svg,xlab:&str,ylab:&str){let x0=L;let x1=W-R;let y0=H-BTM;let y1=T;svg.line(x0,y0,x1,y0,"axis","#111");svg.line(x0,y0,x0,y1,"axis","#111");svg.text((x0+x1)/2.0,H-25.0,xlab,20,"middle");svg.s.push_str(&format!(r#"<text x="27" y="{:.2}" font-size="20" text-anchor="middle" transform="rotate(-90 27 {:.2})">{}</text>"#,(y0+y1)/2.0,(y0+y1)/2.0,ylab));}
fn legend(svg:&mut Svg, items:&[(&str,&str,bool)], x:f64,y:f64){for (i,(name,color,dash)) in items.iter().enumerate(){let yy=y+i as f64*27.0; svg.line(x,yy,x+36.0,yy,if *dash{"curve dash"}else{"curve"},color);svg.text(x+46.0,yy+6.0,name,17,"start");}}

fn relative_noise(out:&Path)->Result<()> {
    let c=0.1f64; let bs=[1.,2.,4.,8.,16.,32.,64.,128.,256.];
    let mut s=Svg::new("Positive-floor single-group noise (c=0.1)"); axes(&mut s,"Replicas B","Relative-variance upper bound");
    let xmin=0f64; let xmax=256f64.log10(); let ymin=(-3.5f64); let ymax=0.5f64;
    for k in -3..=0 {let y=map(k as f64,ymin,ymax,H-BTM,T);s.line(L,y,W-R,y,"grid","#aaa");s.text(L-12.,y+5.,&format!("10^{}",k),15,"end");}
    for (b,lab) in [(1.,"10^0"),(10.,"10^1"),(100.,"10^2")] {let x=map(b.log10(),xmin,xmax,L,W-R);s.line(x,H-BTM,x,T,"grid","#aaa");s.text(x,H-BTM+24.,lab,15,"middle");}
    let colors=["#1f77b4","#ff7f0e","#2ca02c"];
    for (j,&lc) in [0.2f64,0.5,0.9].iter().enumerate(){let pts=bs.iter().map(|b|{let v=(1.-lc)*(lc-c)/(b*lc*lc);(map(b.log10(),xmin,xmax,L,W-R),map(v.log10(),ymin,ymax,H-BTM,T))}).collect::<Vec<_>>();s.poly(&pts,colors[j],false);for p in pts{s.circle(p.0,p.1,colors[j]);}}
    let pts=bs.iter().map(|b|{let v=(1.-c).powi(2)/(4.*b*c);(map(b.log10(),xmin,xmax,L,W-R),map(v.log10(),ymin,ymax,H-BTM,T))}).collect::<Vec<_>>();s.poly(&pts,"#d62728",true);
    legend(&mut s,&[("L_c = 0.2",colors[0],false),("L_c = 0.5",colors[1],false),("L_c = 0.9",colors[2],false),("Uniform bound","#d62728",true)],610.,85.);
    s.finish(&out.join("relative_noise.svg"))
}
fn annealed_noise(out:&Path)->Result<()> {
    let c=0.1f64;let kappa=2.0f64; let mut s=Svg::new("Annealed noise under sufficient B scaling (c=0.1)");axes(&mut s,"Annealing exponent A","Product relative-variance upper bound");
    let xmin=1.;let xmax=64.;let ymin=.39;let ymax=.655;
    for yv in [.40,.45,.50,.55,.60,.65]{let y=map(yv,ymin,ymax,H-BTM,T);s.line(L,y,W-R,y,"grid","#aaa");s.text(L-12.,y+5.,&format!("{yv:.2}"),15,"end");}
    for xv in [1.,10.,20.,30.,40.,50.,60.]{let x=map(xv,xmin,xmax,L,W-R);s.line(x,H-BTM,x,T,"grid","#aaa");s.text(x,H-BTM+24.,&format!("{}",xv as i32),15,"middle");}
    let mut pts=Vec::new();for a in 1..=64{let af=a as f64;let b=(kappa*af*(1.-c).powi(2)/(4.*c)).ceil();let v=(1.-c).powi(2)/(4.*b*c);let p=(1.+v).powf(af)-1.;pts.push((map(af,xmin,xmax,L,W-R),map(p,ymin,ymax,H-BTM,T)));}s.poly(&pts,"#1f77b4",false);
    let lim=(1f64/kappa).exp()-1.;let y=map(lim,ymin,ymax,H-BTM,T);s.line(L,y,W-R,y,"curve dash","#1f77b4");legend(&mut s,&[("exp(1/kappa) - 1","#1f77b4",true)],625.,405.);s.finish(&out.join("annealed_noise.svg"))
}
fn depth_components(out:&Path)->Result<()> {
    let mut s=Svg::new("Width-dependent depth terms (fixed base oracle/rollout depth)");axes(&mut s,"Width","Normalized width-dependent depth");let xmin=0.;let xmax=8.;let ymin=.6;let ymax=9.4;
    for yv in 1..=9{let y=map(yv as f64,ymin,ymax,H-BTM,T);s.line(L,y,W-R,y,"grid","#aaa");s.text(L-12.,y+5.,&yv.to_string(),15,"end");}
    for k in 0..=8{let x=map(k as f64,xmin,xmax,L,W-R);s.line(x,H-BTM,x,T,"grid","#aaa");s.text(x,H-BTM+24.,&format!("2^{}",k),15,"middle");}
    let colors=["#1f77b4","#ff7f0e","#2ca02c","#d62728"];for j in 0..3{let pts=(0..=8).map(|k|(map(k as f64,xmin,xmax,L,W-R),map((k+1) as f64,ymin,ymax,H-BTM,T))).collect::<Vec<_>>();s.poly(&pts,colors[j],false);for p in pts{s.circle(p.0,p.1,colors[j]);}}
    let pts=(0..=8).map(|k|(map(k as f64,xmin,xmax,L,W-R),map(1.,ymin,ymax,H-BTM,T))).collect::<Vec<_>>();s.poly(&pts,colors[3],false);for p in pts{s.circle(p.0,p.1,colors[3]);}
    legend(&mut s,&[("Vary B: reduction",colors[0],false),("Vary A: product reduction",colors[1],false),("Vary Q: cloud selection",colors[2],false),("Vary N: local particle",colors[3],false)],125.,80.);s.finish(&out.join("depth_components.svg"))
}
fn annealing_bound(out:&Path)->Result<()> {
    let mut s=Svg::new("Optimization concentration");axes(&mut s,"Annealing exponent A","Normalized upper-bound term");let xmin=1.;let xmax=40.;let ymin=-6.5;let ymax=0.2;
    for k in -6..=0{let y=map(k as f64,ymin,ymax,H-BTM,T);s.line(L,y,W-R,y,"grid","#aaa");s.text(L-12.,y+5.,&format!("10^{}",k),15,"end");}
    for xv in [1.,5.,10.,15.,20.,25.,30.,35.,40.]{let x=map(xv,xmin,xmax,L,W-R);s.line(x,H-BTM,x,T,"grid","#aaa");s.text(x,H-BTM+24.,&format!("{}",xv as i32),15,"middle");}
    let rs=[0.95f64,0.85,0.70];let colors=["#1f77b4","#ff7f0e","#2ca02c"];for (j,&r) in rs.iter().enumerate(){let pts=(1..=40).map(|a|{let v=r.powi(a);(map(a as f64,xmin,xmax,L,W-R),map(v.log10(),ymin,ymax,H-BTM,T))}).collect::<Vec<_>>();s.poly(&pts,colors[j],false);}legend(&mut s,&[("r_delta = 0.95",colors[0],false),("r_delta = 0.85",colors[1],false),("r_delta = 0.70",colors[2],false)],125.,355.);s.finish(&out.join("annealing_bound.svg"))
}
fn main()->Result<()> {let out=Path::new("figures");create_dir_all(out)?;relative_noise(out)?;annealed_noise(out)?;depth_components(out)?;annealing_bound(out)?;println!("Wrote four MPBRI reproducibility figures to figures/");Ok(())}
