<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>API1</name>
   <tag></tag>
   <elementGuidId>b6b50b9a-d093-4c05-b68c-247bef6f0e0b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>MojoAccessToken</name>
      <type>Main</type>
      <value>eyJ0eXAiOiJKV1QiLCJhbGciOiJFUzI1NiJ9.eyJpc3MiOiJhdXRoLXN0YWdpbmcuam92ZW8uY29tIiwiZXhwIjoiMjAyMC0wMy0xNFQxMDo1NTowMC41ODNaIiwic3ViIjoicmlwcGxlQGpvdmVvLmNvbSJ9.treOaMPzoIlfodWNYthSmSO7swOR0jviJiZi-H5TobEqU_t3E6vSuPFDQH94za4yjNok_PYaL0yKCFxcqbLpUg</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>MojoAgencyId</name>
      <type>Main</type>
      <value>ripple</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>http://mojo2.staging.joveo.com/flash/api/clients?page=1&amp;limit=20&amp;days=This%20month&amp;sortOrder=desc&amp;sortBy=spent&amp;startDate=02/01/2020&amp;endDate=02/13/2020&amp;fields=status,name,resolvedStatus,budgetCap,spent,projectedSpent,clicks,applies,cpa,cta,cpc,startDate,endDate,jobCount,sponsoredJobCount,timezone,markdown,cph,ath,hires,cdSettings&amp;summaryFields=count,clicks,latentClicks,botClicks,applies,applyStarts,cpa,cpc,cta,allCta,spent,jobCount,sponsoredJobCount,allClicks,allBotClicks,allApplies,allApplyStarts,cph,ath,hires&amp;deviceType=all&amp;bidType=all</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


WS.verifyElementPropertyValue(response, 'data.data[0].fields.sponsoredJobCount', '1909')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
