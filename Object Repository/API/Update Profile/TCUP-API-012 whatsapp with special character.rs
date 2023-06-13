<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>TCUP-API-012 whatsapp with special character</name>
   <tag></tag>
   <elementGuidId>105fd3cb-0a56-4c67-ad63-577234668808</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiYWZkZjY4ZDFjM2U5Y2E3YTQ5M2M0MjI0ZDMzMTc2NmU4YjdiNDFlYTNjMTI3NmQ2OGIyNjJkMTQ5MjQ0MzMyMTg5NDU2Y2M3NzQ0Yjg5Y2MiLCJpYXQiOjE2ODY2NzE2OTYuMDUzNDc0LCJuYmYiOjE2ODY2NzE2OTYuMDUzNDc2LCJleHAiOjE3MTgyOTQwOTYuMDUyMjQsInN1YiI6IjU4MiIsInNjb3BlcyI6W119.kTHD7Z30yS9-PYqYV9OiNmqi_KSQHX0xP9otNkS_7zRnEDBCziPFNZq-lYOOEnO1_MzRm0pOQY1UtMujtOVyawumlt9dynkodyObNwyxB30pEUsPeWO5T4HgU-ZgNEQ0JNJO8e-lGutSI-XfHlKuciUWlxGPGIg4j43GBCMoLW7LUgbmYdjuJIq_xEZtfeak6sXAEiCZoCWAKLyIAwq-syldBcf7dmtYKQ5uGZv5U90K2vjb8HtpCgqIB_9O2xhIbFgRsHU329LatVrlpul98edjvNmXgwpNzxE5RQWOUnoOG-v2SovNxGvxUGDJ0Drq6Vizdnf9mj2Li3zT7WdpOtt4-N9nHNs7Rtf-ay5B_kFEvmHYuaDeAoDs0SChHSxOzA_N5UmDLdcuGxfJ2bjXbw_Z4Qr3WFOtOTE-L5ZAvBryZj7wKd0TiyZKgnVuivfahJ-8vqGeRlRDToSBdSQvJD1O4CkF4ja5pCdI0GpWr2Y9M83hvlt9axLFn48sMqu0FINPWe6sOV3Al0kGO9cVSc-FvSFFf5kBKbGPdhoHLoAaIO5Pp4ivOLbQ9183L7Eq3fe6epWFLJZ5sOwOGY6STJW4dwZdVhAIOHxU-tyM0cnCg6v-RFAj0S2BV5spH-l9IAZK9BAMQqzACvWLTBHdXBO9pmsUkyNHvvyPeNVGC54</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;whatsapp&quot;,
      &quot;value&quot;: &quot;+1234567890+&quot;,
      &quot;type&quot;: &quot;Text&quot;,
      &quot;contentType&quot;: &quot;&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
      <webElementGuid>d44e150b-27c0-4218-83b9-612eb7d5deda</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>multipart/form-data</value>
      <webElementGuid>7e2cde4b-d196-499e-83c3-4ecc9d908708</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJhdWQiOiIzIiwianRpIjoiYWZkZjY4ZDFjM2U5Y2E3YTQ5M2M0MjI0ZDMzMTc2NmU4YjdiNDFlYTNjMTI3NmQ2OGIyNjJkMTQ5MjQ0MzMyMTg5NDU2Y2M3NzQ0Yjg5Y2MiLCJpYXQiOjE2ODY2NzE2OTYuMDUzNDc0LCJuYmYiOjE2ODY2NzE2OTYuMDUzNDc2LCJleHAiOjE3MTgyOTQwOTYuMDUyMjQsInN1YiI6IjU4MiIsInNjb3BlcyI6W119.kTHD7Z30yS9-PYqYV9OiNmqi_KSQHX0xP9otNkS_7zRnEDBCziPFNZq-lYOOEnO1_MzRm0pOQY1UtMujtOVyawumlt9dynkodyObNwyxB30pEUsPeWO5T4HgU-ZgNEQ0JNJO8e-lGutSI-XfHlKuciUWlxGPGIg4j43GBCMoLW7LUgbmYdjuJIq_xEZtfeak6sXAEiCZoCWAKLyIAwq-syldBcf7dmtYKQ5uGZv5U90K2vjb8HtpCgqIB_9O2xhIbFgRsHU329LatVrlpul98edjvNmXgwpNzxE5RQWOUnoOG-v2SovNxGvxUGDJ0Drq6Vizdnf9mj2Li3zT7WdpOtt4-N9nHNs7Rtf-ay5B_kFEvmHYuaDeAoDs0SChHSxOzA_N5UmDLdcuGxfJ2bjXbw_Z4Qr3WFOtOTE-L5ZAvBryZj7wKd0TiyZKgnVuivfahJ-8vqGeRlRDToSBdSQvJD1O4CkF4ja5pCdI0GpWr2Y9M83hvlt9axLFn48sMqu0FINPWe6sOV3Al0kGO9cVSc-FvSFFf5kBKbGPdhoHLoAaIO5Pp4ivOLbQ9183L7Eq3fe6epWFLJZ5sOwOGY6STJW4dwZdVhAIOHxU-tyM0cnCg6v-RFAj0S2BV5spH-l9IAZK9BAMQqzACvWLTBHdXBO9pmsUkyNHvvyPeNVGC54</value>
      <webElementGuid>12d892f9-c2bf-4745-9b35-fef742fbeba2</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://demo-app.coding.id/api/updateprofile</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>