<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>html_Administator Login                    _667272</name>
   <tag></tag>
   <elementGuidId>d086bfa6-f8fc-4c90-b5fa-cf93bd05c514</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>html</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*/text()[normalize-space(.)='']/parent::*</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>html</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>lang</name>
      <type>Main</type>
      <value>en</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
    
    
    
    Administator Login
    
    
    
    
    
    
    
    
    
    
  
label {width:100%}
input { font-weight: 400; }


  .alert-inverse {position: absolute; margin-top: -270px; margin-left: -30px; width: 350px; height: 300px; text-align: center; padding-top: 124px; text-transform: uppercase; font-weight: bold; font-size: 16px; letter-spacing: 2px; background-color: rgba(19, 19, 19, 0.89); color: #ffffff; }
  .form-signin .form-control { height: 50px; border-radius: 3px;position: relative; -webkit-box-sizing: border-box; box-sizing: border-box; margin-bottom: 10px!important; border: 0px; border: 1px solid #9d9d9d; font-size: 16px; }
  .form-signin .btn-primary { height: 50px; border-radius: 3px !important;font-size: 14px; letter-spacing: 4px; }
  .btn-primary { background: #0031bc }
   body { background:#ffffff !important }
    .matrialprogress{position:relative;height:10px;display:block;width:100%;background-color:#bfc1ce;border-radius:2px;background-clip:padding-box;margin:.5rem 0 1rem 0;overflow:hidden}
    .matrialprogress .determinate{position:absolute;background-color:inherit;top:0;bottom:0;background-color:#3f51b5;transition:width .3s linear}
    .matrialprogress .indeterminate{background-color:#3f51b5}
    .matrialprogress .indeterminate:before{content:'';position:absolute;background-color:inherit;top:0;left:0;bottom:0;will-change:left,right;-webkit-animation:indeterminate 2.1s cubic-bezier(0.65,0.815,0.735,0.395) infinite;animation:indeterminate 2.1s cubic-bezier(0.65,0.815,0.735,0.395) infinite}
    .matrialprogress .indeterminate:after{content:'';position:absolute;background-color:inherit;top:0;left:0;bottom:0;will-change:left,right;-webkit-animation:indeterminate-short 2.1s cubic-bezier(0.165,0.84,0.44,1) infinite;animation:indeterminate-short 2.1s cubic-bezier(0.165,0.84,0.44,1) infinite;-webkit-animation-delay:1.15s;animation-delay:1.15s}
    @-webkit-keyframes indeterminate{0%{left:-35%;right:100%}
    60%{left:100%;right:-90%}
    100%{left:100%;right:-90%}
    }@keyframes indeterminate{0%{left:-35%;right:100%}
    60%{left:100%;right:-90%}
    100%{left:100%;right:-90%}
    }@-webkit-keyframes indeterminate-short{0%{left:-200%;right:100%}
    60%{left:107%;right:-8%}
    100%{left:107%;right:-8%}
    }@keyframes indeterminate-short{0%{left:-200%;right:100%}
    60%{left:107%;right:-8%}
    100%{left:107%;right:-8%}
    }
body {
    background: #000000
}
body:after {
    content:'';
    position: fixed;
    height: 50%;
    width: 100%;
    top: 50%;
    bottom: 50%;
    border-top: 2px solid #000daa;
    background-image: radial-gradient(circle farthest-side at center bottom,#3056d3,#02269e 125%);
    z-index:1;
}
  input { width: 100%; height: 44px; padding: 10px; margin-bottom: 10px; color: #000 !important; font-weight: bold }
  
    $(function() {
      Login.init()
    });
  
    $(function () {
    $(&quot;.form-signin&quot;).on('submit',function(){
    $(&quot;.resultlogin&quot;).html(&quot;&lt;div class='alert alert-inverse loading wow fadeOut animated'>Hold On...&lt;/div>&quot;);
    $.post(&quot;https://www.phptravels.net/admin/login&quot;,$(&quot;.form-signin&quot;).serialize(), function(response){
    var resp = $.parseJSON(response);
    console.log(resp);
    if(!resp.status){
    $(&quot;.resultlogin&quot;).html(&quot;&lt;div class='alert alert-danger loading wow fadeIn animated'>&quot;+resp.msg+&quot;&lt;/div>&quot;);
    }else{
    $(&quot;.resultlogin&quot;).html(&quot;&lt;div class='alert alert-success login wow fadeIn animated'>Redirecting Please Wait...&lt;/div>&quot;);
    window.location.replace(resp.url);
    }
    }); });
    $(&quot;.resetbtn&quot;).on('click',function(){
    var resetemail = $(&quot;#resetemail&quot;).val();
    if(resetemail == &quot;&quot;){
    alert(&quot;Please Enter Email Address&quot;);
    }else{
     $(&quot;.resultreset&quot;).html(&quot;&lt;div class='matrialprogress'>&lt;div class='indeterminate'>&lt;/div>&lt;/div>&quot;);
     $.post(&quot;https://www.phptravels.net/admin/resetpass&quot;,$(&quot;#passresetfrm&quot;).serialize(), function(response){
     if($.trim(response) == '1'){
     $(&quot;.resultreset&quot;).html(&quot;&lt;div class='alert alert-success'>New Password sent to &quot;+resetemail+&quot;, Kindly check email.&lt;/div>&quot;);
     }else{
     $(&quot;.resultreset&quot;).html(&quot;&lt;div class='alert alert-danger'>Email Not Found&lt;/div>&quot;);
     } });
     }
     });
     });
  #katalon{font-family:monospace;font-size:13px;background-color:rgba(0,0,0,.7);position:fixed;top:0;left:0;right:0;display:block;z-index:999999999;line-height: normal} #katalon div{padding:0;margin:0;color:#fff;} #katalon kbd{display:inline-block;padding:3px 5px;font:13px Consolas,&quot;Liberation Mono&quot;,Menlo,Courier,monospace;line-height:10px;color:#555;vertical-align:middle;background-color:#fcfcfc;border:1px solid #ccc;border-bottom-color:#bbb;border-radius:3px;box-shadow:inset 0 -1px 0 #bbb;font-weight: bold} div#katalon-spy_elementInfoDiv {color: lightblue; padding: 0px 5px 5px} div#katalon-spy_instructionDiv {padding: 5px 5px 2.5px}
  

  
  
   
   
(function(w,d,s,l,i){w[l]=w[l]||[];w[l].push({'gtm.start':
new Date().getTime(),event:'gtm.js'});var f=d.getElementsByTagName(s)[0],
j=d.createElement(s),dl=l!='dataLayer'?'&amp;l='+l:'';j.async=true;j.src=
'https://www.googletagmanager.com/gtm.js?id='+i+dl;f.parentNode.insertBefore(j,f);
})(window,document,'script','dataLayer','GTM-MPDFM5X');



.imagelogo { width: 170px; height: 40px; background-image: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAATYAAABQCAYAAACTZllaAAAACXBIWXMAAAsTAAALEwEAmpwYAAAJ22lUWHRYTUw6Y29tLmFkb2JlLnhtcAAAAAAAPD94cGFja2V0IGJlZ2luPSLvu78iIGlkPSJXNU0wTXBDZWhpSHpyZVN6TlRjemtjOWQiPz4gPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRhLyIgeDp4bXB0az0iQWRvYmUgWE1QIENvcmUgNS42LWMxNDIgNzkuMTYwOTI0LCAyMDE3LzA3LzEzLTAxOjA2OjM5ICAgICAgICAiPiA8cmRmOlJERiB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPiA8cmRmOkRlc2NyaXB0aW9uIHJkZjphYm91dD0iIiB4bWxuczp4bXA9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC8iIHhtbG5zOmRjPSJodHRwOi8vcHVybC5vcmcvZGMvZWxlbWVudHMvMS4xLyIgeG1sbnM6cGhvdG9zaG9wPSJodHRwOi8vbnMuYWRvYmUuY29tL3Bob3Rvc2hvcC8xLjAvIiB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RFdnQ9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZUV2ZW50IyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5jb20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENDIChXaW5kb3dzKSIgeG1wOkNyZWF0ZURhdGU9IjIwMTktMDgtMDhUMjI6MTc6MjMrMDU6MDAiIHhtcDpNb2RpZnlEYXRlPSIyMDE5LTA5LTIwVDAzOjEyOjQ3KzA1OjAwIiB4bXA6TWV0YWRhdGFEYXRlPSIyMDE5LTA5LTIwVDAzOjEyOjQ3KzA1OjAwIiBkYzpmb3JtYXQ9ImltYWdlL3BuZyIgcGhvdG9zaG9wOkNvbG9yTW9kZT0iMyIgcGhvdG9zaG9wOklDQ1Byb2ZpbGU9InNSR0IgSUVDNjE5NjYtMi4xIiB4bXBNTTpJbnN0YW5jZUlEPSJ4bXAuaWlkOjZkYzg5ZWNjLTE5MjItZTc0OS1hNmIxLWZmMDE3YTgwZTE0ZCIgeG1wTU06RG9jdW1lbnRJRD0iYWRvYmU6ZG9jaWQ6cGhvdG9zaG9wOjY5NzM0MTlkLTY3OGMtOTE0OC1iOWVlLWE1ZjY4MzI0ZmQxNSIgeG1wTU06T3JpZ2luYWxEb2N1bWVudElEPSJ4bXAuZGlkOmE1YTdiZTcwLTRlMDUtZTk0My1hZTBjLWFjNDRiM2QzN2I3OSI+IDx4bXBNTTpIaXN0b3J5PiA8cmRmOlNlcT4gPHJkZjpsaSBzdEV2dDphY3Rpb249ImNyZWF0ZWQiIHN0RXZ0Omluc3RhbmNlSUQ9InhtcC5paWQ6YTVhN2JlNzAtNGUwNS1lOTQzLWFlMGMtYWM0NGIzZDM3Yjc5IiBzdEV2dDp3aGVuPSIyMDE5LTA4LTA4VDIyOjE3OjIzKzA1OjAwIiBzdEV2dDpzb2Z0d2FyZUFnZW50PSJBZG9iZSBQaG90b3Nob3AgQ0MgKFdpbmRvd3MpIi8+IDxyZGY6bGkgc3RFdnQ6YWN0aW9uPSJjb252ZXJ0ZWQiIHN0RXZ0OnBhcmFtZXRlcnM9ImZyb20gaW1hZ2UvcG5nIHRvIGFwcGxpY2F0aW9uL3ZuZC5hZG9iZS5waG90b3Nob3AiLz4gPHJkZjpsaSBzdEV2dDphY3Rpb249InNhdmVkIiBzdEV2dDppbnN0YW5jZUlEPSJ4bXAuaWlkOjJmODBmMzJkLTdkMWMtYjE0OC04OTRmLTcyMGMwZjBhMTdkYiIgc3RFdnQ6d2hlbj0iMjAxOS0wOS0yMFQwMzoxMTo1OCswNTowMCIgc3RFdnQ6c29mdHdhcmVBZ2VudD0iQWRvYmUgUGhvdG9zaG9wIENDIChXaW5kb3dzKSIgc3RFdnQ6Y2hhbmdlZD0iLyIvPiA8cmRmOmxpIHN0RXZ0OmFjdGlvbj0ic2F2ZWQiIHN0RXZ0Omluc3RhbmNlSUQ9InhtcC5paWQ6NmMxODExMGItN2Q5Ni02YjQ2LTk0NGItNWZkNTU0OWVhMTExIiBzdEV2dDp3aGVuPSIyMDE5LTA5LTIwVDAzOjEyOjQ3KzA1OjAwIiBzdEV2dDpzb2Z0d2FyZUFnZW50PSJBZG9iZSBQaG90b3Nob3AgQ0MgKFdpbmRvd3MpIiBzdEV2dDpjaGFuZ2VkPSIvIi8+IDxyZGY6bGkgc3RFdnQ6YWN0aW9uPSJjb252ZXJ0ZWQiIHN0RXZ0OnBhcmFtZXRlcnM9ImZyb20gYXBwbGljYXRpb24vdm5kLmFkb2JlLnBob3Rvc2hvcCB0byBpbWFnZS9wbmciLz4gPHJkZjpsaSBzdEV2dDphY3Rpb249ImRlcml2ZWQiIHN0RXZ0OnBhcmFtZXRlcnM9ImNvbnZlcnRlZCBmcm9tIGFwcGxpY2F0aW9uL3ZuZC5hZG9iZS5waG90b3Nob3AgdG8gaW1hZ2UvcG5nIi8+IDxyZGY6bGkgc3RFdnQ6YWN0aW9uPSJzYXZlZCIgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDo2ZGM4OWVjYy0xOTIyLWU3NDktYTZiMS1mZjAxN2E4MGUxNGQiIHN0RXZ0OndoZW49IjIwMTktMDktMjBUMDM6MTI6NDcrMDU6MDAiIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkFkb2JlIFBob3Rvc2hvcCBDQyAoV2luZG93cykiIHN0RXZ0OmNoYW5nZWQ9Ii8iLz4gPC9yZGY6U2VxPiA8L3htcE1NOkhpc3Rvcnk+IDx4bXBNTTpEZXJpdmVkRnJvbSBzdFJlZjppbnN0YW5jZUlEPSJ4bXAuaWlkOjZjMTgxMTBiLTdkOTYtNmI0Ni05NDRiLTVmZDU1NDllYTExMSIgc3RSZWY6ZG9jdW1lbnRJRD0ieG1wLmRpZDphNWE3YmU3MC00ZTA1LWU5NDMtYWUwYy1hYzQ0YjNkMzdiNzkiIHN0UmVmOm9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDphNWE3YmU3MC00ZTA1LWU5NDMtYWUwYy1hYzQ0YjNkMzdiNzkiLz4gPC9yZGY6RGVzY3JpcHRpb24+IDwvcmRmOlJERj4gPC94OnhtcG1ldGE+IDw/eHBhY2tldCBlbmQ9InIiPz5BxDvzAAAhyElEQVR4nO2deZgU1bn/P9U907My7CAiuADuW9xRiV53XNAkalwvUa+JSojELRo1xhtNvJIYUTQucYsXYzQkStQblaAx3hDcFxRkEwRBdoYZZunp7rp/fKt+XV1VvQ09C/M7n+fpZ6arazl1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); *background-image: url(); background-size: 170px 40px; float:left; }











Selenium, Bots, or any automated softwares are not allowed to test here!



Buy Now







&lt;iframe src=&quot;https://www.googletagmanager.com/ns.html?id=GTM-MPDFM5X&quot;
height=&quot;0&quot; width=&quot;0&quot; style=&quot;display:none;visibility:hidden&quot;>&lt;/iframe>





    window.__lc = window.__lc || {};
    window.__lc.license = 4618001;
    (function() {
        var lc = document.createElement('script'); lc.type = 'text/javascript'; lc.async = true;
        lc.src = ('https:' == document.location.protocol ? 'https://' : 'http://') + 'cdn.livechatinc.com/tracking.js';
        var s = document.getElementsByTagName('script')[0]; s.parentNode.insertBefore(lc, s);
    })();


   
    
    
      
        Login Panel
       
        
      
      
        
      
       
          
            
             Remember me 
          
          
            
               Forget Password
              
            
          
        
      
      Login
      
    
    
       Forgot Password
      
      Enter your email address to reset your password
      
      
        
        
        
      
      
      
         Back
        Reset My Password
      
    
    
        Powered by  PHPTRAVELS v7.4 
   

    
    Ladda.bind( 'div:not(.progress-demo) button', { timeout: 2000 } );
    Ladda.bind( '.progress-demo button', {
    	callback: function( instance ) {
    		var progress = 0;
    		var interval = setInterval( function() {
    			progress = Math.min( progress + Math.random() * 0.1, 1 );
    			instance.setProgress( progress );
    			if( progress === 1 ) {
    				instance.stop();
    				clearInterval( interval );
    			}
    		}, 200 );
    	}
    } );
  
  
  
  
  
    var cb, optionSet1;
        $(&quot;.checkbox&quot;).iCheck({
          checkboxClass: &quot;icheckbox_square-grey&quot;,
          radioClass: &quot;iradio_square-grey&quot;
        });

        $(&quot;.radio&quot;).iCheck({
          checkboxClass: &quot;icheckbox_square-grey&quot;,
          radioClass: &quot;iradio_square-grey&quot;
        });
  

  
  
    new WOW().init();
  
  /html[1]window.__lo_site_id=193429;(function(){var a=document.createElement(&quot;script&quot;);a.type=&quot;text/javascript&quot;;a.async=!0;a.src=&quot;https://d10lpsik1i8c69.cloudfront.net/w.js&quot;;var b=document.getElementsByTagName(&quot;script&quot;)[0];b.parentNode.insertBefore(a,b)})();</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>//*/text()[normalize-space(.)='']/parent::*</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//html</value>
   </webElementXpaths>
</WebElementEntity>
